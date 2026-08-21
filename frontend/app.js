"use strict";

/* ============================================================================
 * protobuf + gRPC-Web client
 *
 * Schema is loaded from the real proto/wavelet.proto at runtime (served by
 * main.rs at /proto/wavelet.proto) and parsed with protobuf.js, rather than
 * hand-rolling the wire format against a copied-down field list. That keeps
 * this file in sync with the server automatically instead of needing to be
 * hand-edited every time the schema changes.
 *
 * Current server contract (see proto/wavelet.proto, cwt.rs):
 *   package wavelet;
 *   service ProcessingService { rpc ProcessAudio(AudioUploadRequest) returns (WaveletResult); }
 *   message AudioUploadRequest { bytes audio_data = 1; string name = 2; }
 *   message WaveletResult {
 *     bytes image = 1; uint32 sample_rate = 2; uint32 n_samples = 3;
 *     string codec = 4; uint32 width = 5; uint32 height = 6; string error = 7;
 *   }
 * ========================================================================== */

const MAX_UPLOAD_MB = 5;
const MAX_UPLOAD_BYTES = MAX_UPLOAD_MB * 1024 * 1024; // Mirrors MAX_UPLOAD_BYTES in cwt.rs

const PROTO_URL = "/proto/wavelet.proto";
const SERVICE_PATH = "/wavelet.ProcessingService/ProcessAudio";

let AudioUploadRequest = null;
let WaveletResult = null;

async function loadProto() {
  const resp = await fetch(PROTO_URL);
  if (!resp.ok) {
    throw new Error(
      `Could not load ${PROTO_URL} (HTTP ${resp.status}). ` +
      `Make sure main.rs is serving proto/ at /proto.`
    );
  }
  const src = await resp.text();
  const root = protobuf.parse(src).root;
  AudioUploadRequest = root.lookupType("wavelet.AudioUploadRequest");
  WaveletResult = root.lookupType("wavelet.WaveletResult");
}

// ---- gRPC-Web framing: [flags:1][length:4 BE][message bytes] --------------

function concatBytes(chunks) {
  const total = chunks.reduce((n, c) => n + c.length, 0);
  const out = new Uint8Array(total);
  let offset = 0;
  for (const c of chunks) {
    out.set(c, offset);
    offset += c.length;
  }
  return out;
}

function frameMessage(messageBytes) {
  const header = new Uint8Array(5);
  header[0] = 0x00; // 0 = uncompressed data frame
  new DataView(header.buffer).setUint32(1, messageBytes.length, false);
  return concatBytes([header, messageBytes]);
}

function parseFrames(buf) {
  const frames = [];
  let pos = 0;
  while (pos < buf.length) {
    const flags = buf[pos];
    const length = new DataView(buf.buffer, buf.byteOffset + pos + 1, 4).getUint32(0, false);
    const start = pos + 5;
    const end = start + length;
    frames.push({ flags, bytes: buf.subarray(start, end) });
    pos = end;
  }
  return frames;
}

/**
 * Calls wavelet.ProcessingService/ProcessAudio over gRPC-Web and resolves
 * with a decoded WaveletResult (plain object), or rejects with an Error
 * carrying the gRPC/application error message.
 */
async function processAudio(audioBytes, filename) {
  if (!AudioUploadRequest || !WaveletResult) {
    throw new Error("Schema not loaded yet — try again in a moment.");
  }

  const requestMessage = AudioUploadRequest.create({ audioData: audioBytes, name: filename });
  const body = frameMessage(AudioUploadRequest.encode(requestMessage).finish());

  const res = await fetch(SERVICE_PATH, {
    method: "POST",
    headers: {
      "content-type": "application/grpc-web+proto",
      "x-grpc-web": "1",
    },
    body,
  });

  const raw = new Uint8Array(await res.arrayBuffer());

  const grpcStatusHeader = res.headers.get("grpc-status");
  if (grpcStatusHeader && grpcStatusHeader !== "0") {
    throw new Error(res.headers.get("grpc-message") || `gRPC error ${grpcStatusHeader}`);
  }

  const frames = parseFrames(raw);
  const dataFrame = frames.find((f) => (f.flags & 0x80) === 0);
  const trailerFrame = frames.find((f) => (f.flags & 0x80) !== 0);

  if (trailerFrame) {
    const trailerText = new TextDecoder().decode(trailerFrame.bytes);
    const statusMatch = /grpc-status:\s*(\d+)/i.exec(trailerText);
    if (statusMatch && statusMatch[1] !== "0") {
      const msgMatch = /grpc-message:\s*([^\r\n]+)/i.exec(trailerText);
      throw new Error(decodeURIComponent(msgMatch ? msgMatch[1] : "request failed"));
    }
  }

  if (!dataFrame) {
    throw new Error("server returned no data frame");
  }

  const decoded = WaveletResult.decode(dataFrame.bytes);
  return {
    pngImage: decoded.image || new Uint8Array(0),
    sampleRate: decoded.sampleRate || 0,
    nSamples: decoded.nSamples || 0,
    codec: decoded.codec || "",
    width: decoded.width || 0,
    height: decoded.height || 0,
    error: decoded.error || "",
  };
}

/* ============================================================================
 * UI wiring
 * ========================================================================== */


const dropzone = document.getElementById("dropzone");
const fileInput = document.getElementById("file-input");
const dzTitle = document.getElementById("dz-title");
const dzSub = document.getElementById("dz-sub");
const dzFile = document.getElementById("dz-file");
const analyzeBtn = document.getElementById("analyze-btn");
const statusEl = document.getElementById("status");
const outputSection = document.getElementById("output");
const readoutEl = document.getElementById("readout");
const imgEl = document.getElementById("scalogram-img");

let selectedFile = null;
let schemaReady = false;

function updateAnalyzeAvailability() {
  analyzeBtn.disabled = !(selectedFile && schemaReady);
}

function formatBytes(n) {
  if (n < 1024) return `${n} B`;
  if (n < 1024 * 1024) return `${(n / 1024).toFixed(1)} KB`;
  return `${(n / (1024 * 1024)).toFixed(2)} MB`;
}

function setStatus(message, isError = false) {
  statusEl.textContent = message;
  statusEl.classList.toggle("error", isError);
}

function selectFile(file) {
  if (!file) return;
  if (file.size > MAX_UPLOAD_BYTES) {
    setStatus(`"${file.name}" is ${formatBytes(file.size)}, over the 5MB limit.`, true);
    selectedFile = null;
    updateAnalyzeAvailability();
    dzFile.hidden = true;
    return;
  }
  selectedFile = file;
  dzFile.hidden = false;
  dzFile.textContent = `${file.name} \u00b7 ${formatBytes(file.size)}`;
  dzTitle.textContent = "Ready to analyze";
  dzSub.textContent = "Click Analyze audio below, or drop a different file to replace it";
  updateAnalyzeAvailability();
  setStatus(schemaReady ? "" : "Loading schema…");
}

dropzone.addEventListener("click", () => fileInput.click());
dropzone.addEventListener("keydown", (e) => {
  if (e.key === "Enter" || e.key === " ") {
    e.preventDefault();
    fileInput.click();
  }
});

fileInput.addEventListener("change", () => selectFile(fileInput.files[0]));

["dragenter", "dragover"].forEach((evt) =>
  dropzone.addEventListener(evt, (e) => {
    e.preventDefault();
    dropzone.classList.add("drag-over");
  })
);
["dragleave", "drop"].forEach((evt) =>
  dropzone.addEventListener(evt, (e) => {
    e.preventDefault();
    dropzone.classList.remove("drag-over");
  })
);
dropzone.addEventListener("drop", (e) => {
  const file = e.dataTransfer.files && e.dataTransfer.files[0];
  selectFile(file);
});

analyzeBtn.addEventListener("click", async () => {
  if (!selectedFile) return;

  analyzeBtn.disabled = true;
  outputSection.hidden = true;
  setStatus(`Uploading and analyzing "${selectedFile.name}"...`);

  try {
    const bytes = new Uint8Array(await selectedFile.arrayBuffer());
    const result = await processAudio(bytes, selectedFile.name);

    if (!result.pngImage.length) {
      throw new Error("no scalogram image returned");
    }

    const blob = new Blob([result.pngImage], { type: "image/png" });
    imgEl.src = URL.createObjectURL(blob);

    const durationSec = result.sampleRate ? (result.nSamples / result.sampleRate).toFixed(2) : "?";
    readoutEl.innerHTML = `
      <span><span class="k">codec</span> ${result.codec || "?"}</span>
      <span><span class="k">sample rate</span> ${result.sampleRate || "?"} Hz</span>
      <span><span class="k">samples</span> ${result.nSamples || "?"}</span>
      <span><span class="k">duration</span> ${durationSec} s</span>
      <span><span class="k">image</span> ${result.width}&times;${result.height}</span>
    `;

    outputSection.hidden = false;
    setStatus("Done.");
  } catch (err) {
    setStatus(err.message || "Something went wrong processing that file.", true);
  } finally {
    updateAnalyzeAvailability();
  }
});

// ---- schema bootstrap -------------------------------------------------

loadProto()
  .then(() => {
    schemaReady = true;
    updateAnalyzeAvailability();
    if (selectedFile) setStatus("");
  })
  .catch((err) => {
    console.error(err);
    setStatus(err.message || "Could not load the wavelet schema.", true);
  });