from functools import partial
import os
import io

import jax
import jax.numpy as jnp
import matplotlib.pyplot as plt
import numpy as np
import soundfile as sf

###
# Morse Wavelet computation
###

def morse_log_L1_normalization(beta: float, gamma: float) -> jnp.ndarray:
    """
    Calculate the log of bandwidth (L1) normalization constant to the Morse Wavelet
    """
    return jnp.log(2.0) + (beta/gamma)*(1.0 + jnp.log(gamma) - jnp.log(beta))


def morse_peak_frequency(beta: float, gamma: float) -> jnp.ndarray:
    """
    Peak Morse Wavelet frequency
    """
    return (beta/gamma)**(1.0/gamma)

def morse_wavelet_freq_L1(omega: jnp.ndarray, beta: float, gamma: float) -> jnp.ndarray:
    """
    Morse Wavelet in frequency domain under L1 normalization.
    """
    log_norm_const = morse_log_L1_normalization(beta, gamma)
    analytic_omega = jnp.where(omega > 0, omega, 1.0)
    log_mag = log_norm_const + beta * jnp.log(analytic_omega) - analytic_omega**gamma
    mag = jnp.exp(log_mag)
    return jnp.where(omega > 0, mag, 0.0)


@partial(jax.jit, static_argnames=())
def morse_transform_L1(signal: jnp.ndarray, scales: jnp.ndarray, 
                    beta: float = 3.0, gamma: float = 60.0, fs: float = 1.0) -> jnp.ndarray:
    """
    Continuous Wavelet Transformation of the signal based on Morse mother Wavelet
    """
    # Frequency domain setup
    n = signal.shape[-1]
    x_hat = jnp.fft.fft(signal)
    omega = (2.0*jnp.pi) * jnp.fft.fftfreq(n, d=1.0/fs)

    # Transform 
    def _transform_at_scale(s: jnp.float32):
        psi = morse_wavelet_freq_L1(s * omega, beta, gamma)
        return jnp.fft.ifft(x_hat * psi)

    return jax.vmap(_transform_at_scale)(scales)


###
# Domain helper
###

def scale_to_hz(scale: jnp.ndarray, beta: float, gamma: float) -> jnp.ndarray:
    omega_peak = morse_peak_frequency(beta, gamma)
    return omega_peak / (2.0 * jnp.pi * scale)


def hz_to_scale(freq_hz: jnp.ndarray, beta: float, gamma: float) -> jnp.ndarray:
    omega_peak = morse_peak_frequency(beta, gamma)
    return omega_peak / (2.0 * jnp.pi * freq_hz)


def log_scales(n_scales: int, f_min: float, f_max: float, beta: float = 3.0, gamma: float = 60.0,) -> jnp.ndarray:
    freqs = jnp.geomspace(f_min, f_max, n_scales)
    return hz_to_scale(freqs, beta, gamma)


###
# Scalogram
###

def scalogram(
    signal: np.ndarray,
    sr: int,
    f_min: float = 20.0,
    f_max: float = 15000.0,
    n_scales: int = 128,
    beta: float = 3.0,
    gamma: float = 60.0,
    title: str = "Audio Scalogram",
    out_path: str = None,
):
    scales = log_scales(n_scales, f_min, f_max, beta=beta, gamma=gamma)
    freqs = np.asarray(scale_to_hz(scales, beta, gamma))

    W = morse_transform_L1(
        jnp.asarray(signal),
        scales.astype(jnp.float32),
        beta=beta,
        gamma=gamma,
        fs=float(sr),
    )

    mag = np.asarray(jnp.abs(W))
    t = np.arange(signal.shape[0]) / float(sr)

    fig, ax = plt.subplots(figsize=(10, 5))
    im = ax.pcolormesh(t, freqs, mag, shading="auto", cmap="magma")
    ax.set_yscale("log")
    ax.set_xlabel("Time (s)")
    ax.set_ylabel("Frequency (Hz)")
    ax.set_title(title)
    fig.colorbar(im, ax=ax, label="|W|")
    fig.tight_layout()

    if return_bytes:
        dpi = 250
        buf = io.BytesIO()
        fig.savefig(buf, format="png", dpi=dpi)
        width_px, height_px = fig.get_size_inches() * dpi
        plt.close(fig)
        return buf.getvalue(), int(width_px), int(height_px)

    if out_path:
        plt.savefig(out_path, dpi=250)
        plt.close(fig)
    else:
        plt.show()

