"""
Generate synthetic WAV test audio signals and save them 
"""

import os
import wave
import math
import struct

#%%
SAMPLE_RATE = 44100
DATA_DIR = os.path.join(os.path.dirname(__file__), "../data")

#%%
def write_wav(filename: str, samples: list[float], sample_rate: int = SAMPLE_RATE):
    """
    Writing synthetic data to a WAV file
    """
    # Setting output path
    path = os.path.join(DATA_DIR, filename)

    # Clamp and convert to PCM-16
    pcm = [max(-32768, min(32767, int(s * 32767))) for s in samples]
    with wave.open(path, "w") as f:
        f.setnchannels(1)
        f.setsampwidth(2)
        f.setframerate(sample_rate)
        f.writeframes(struct.pack(f"<{len(pcm)}h", *pcm))
    print(f"  Wrote {path} ({len(samples)/sample_rate:.2f}s)")


#%%
def linear_chirp(count: int, f_low: float, f_high: float, sample_rate: int = SAMPLE_RATE) -> list[float]:
    """
    Linear chirp signal generation
    """
    t_end = count / sample_rate
    return [
        math.sin(2.0 * math.pi * (f_low * i / sample_rate + (f_high - f_low) / (2.0 * t_end) * (i / sample_rate) ** 2.0))
        for i in range(count)
    ]

#%%
if __name__ == "__main__":
    # Writing 10 seconds of chirp from 0 Hz to 10K Hz
    write_wav("chirp_20_10000hz.wav", linear_chirp(SAMPLE_RATE * 10, 20.0, 10000.0))
