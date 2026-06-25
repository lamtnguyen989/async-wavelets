#!/usr/bin/env python3

"""
Quick visualization script
"""

import sys
import numpy as np
import soundfile as sf
import matplotlib.pyplot as plt

if len(sys.argv) != 2:
    print(f"Usage: python {sys.argv[0]} <audio file>")
    sys.exit(1)

filename = sys.argv[1]

# Read audio file
audio, sample_rate = sf.read(filename)

# Metadata
info = sf.info(filename)
print(f"Sample rate : {info.samplerate} Hz")
print(f"Channels    : {info.channels}")
print(f"Frames      : {info.frames}")
print(f"Duration    : {info.duration:.2f} s")
print(f"Subtype     : {info.subtype}")

# Convert stereo to mono if needed
if audio.ndim > 1:
    audio = audio.mean(axis=1)

# Time axis
time = np.arange(len(audio)) / sample_rate

# FFT
fft = np.fft.rfft(audio)
freq = np.fft.rfftfreq(len(audio), 1 / sample_rate)
magnitude = np.abs(fft)

# Plot
plt.figure(figsize=(12, 8))

# Waveform
plt.subplot(2, 1, 1)
plt.plot(time, audio)
plt.title("Waveform")
plt.xlabel("Time (s)")
plt.ylabel("Amplitude")

# Frequency spectrum
plt.subplot(2, 1, 2)
plt.plot(freq, magnitude)
plt.title("Frequency Spectrum")
plt.xlabel("Frequency (Hz)")
plt.ylabel("Magnitude")
plt.xlim(0, sample_rate / 2)

plt.tight_layout()
plt.show()
