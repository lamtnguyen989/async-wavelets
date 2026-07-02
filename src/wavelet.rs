use num_complex::{Complex64};
use statrs::function::gamma::gamma as gamma_function;

/***
*   Behavior macros
***/

/// Normalization constant metric context
 #[derive(Clone, Copy, Debug)]
pub enum Normalization {
    L1, // Convention to use this
    L2
}

/// Wave coefficient type (i.e. the type of values that will be computed for scalograms)
#[derive(Debug, Clone)]
pub enum WaveCoefficientType
{
    Magnitude,  // |W(t,s)|
    Power,      // |W(t,s)|^2
}

/***
*   Generalized Morse Wavelet
***/

/// Parameters deciding which wavelet within the Generalized Morse Wavelet family
#[derive(Clone, Copy, Debug)]
pub struct GmwParams
{
    pub gamma:  f64,
    pub beta:   f64,
}

impl GmwParams
{
    /// Constructor
    pub fn new(beta: f64, gamma: f64) -> Self {
        // Parameters bounds assert (error checking impl later)
        assert!(beta >= 0.0 && gamma >= 0.0, "Both beta and gamma parameters must be positive!");

        return Self {
            beta:   beta,
            gamma:  gamma
        }
    }

    /// Peak frequency of GMW based on the parameters
    pub fn peak_freq(&self) -> f64 {return (self.beta/self.gamma).powf(1.0/self.gamma);}

    /// Calculate normalization constant based on metric context
    pub fn normalization_const(&self, norm_type: Normalization) -> f64 {
        let (beta, gamma): (f64, f64) = (self.beta, self.gamma);

        match norm_type {
            Normalization::L1 => {
                let c = 2.0;    // Convention
                return c * (std::f64::consts::E*gamma / beta).powf(beta/gamma);
            },
            Normalization::L2 => {
                let z: f64 = (2.0*beta + 1.0) / gamma;
                let norm_const_squared: f64 = (2.0*std::f64::consts::PI*gamma*2.0_f64.powf(z)) / gamma_function(z);
                return norm_const_squared.sqrt();
            },
        }
    }
}

impl From<(f64, f64)> for GmwParams {
    fn from(tuple: (f64, f64)) -> Self {
        let (beta, gamma) = (tuple.0, tuple.1);
        return GmwParams::new(beta, gamma);
    }
}

/// Actual Generalized Morse wavelet handler
#[derive(Clone, Copy, Debug)]
pub struct GeneralizedMorseWavelet
{
    pub params: GmwParams,  // Parameters
    alpha: f64,             // Normalization constant
    scale_exp: f64,         // Scale factor based on metric context for computing coeffcient values 
}

impl GeneralizedMorseWavelet
{
    /// Constructors
    pub fn new<P>(params: P, norm_type: Normalization) -> Self
    where P: Into<GmwParams>
    {
        let params: GmwParams = params.into();
        let scale_exp = match norm_type {
            Normalization::L1 => 0.0,
            Normalization::L2 => 0.5,
        };

        return Self {
            params:         params,
            alpha:          params.normalization_const(norm_type),
            scale_exp:      scale_exp,
        }
    }

    /// Wavelet transform coefficient value via convolution handled in frequency space
    /// Technically, this is complex-valued, but with real parameters, this reduced to real-valued
    pub fn freq_coefficient_value(&self, omega: f64, scale: f64) -> f64 {
        // Heaviside step function in action
        if omega <= 0.0 {
            return 0.0;
        }

        // Scale angular frequency
        let s_omega = scale * omega;

        // Calculate unscaled value
        let mut wavelet_value = s_omega.powf(self.params.beta) * f64::exp(-s_omega.powf(self.params.gamma));
        wavelet_value *= self.alpha;    // Normalize the wavelet value

        // Scale the coefficient value based on the metric context
        match self.scale_exp {
            0.5 => {wavelet_value *= scale.sqrt();},
            0.0 => {},
            _   => {wavelet_value *= scale.powf(self.scale_exp);}
        }
        
        // Morse Wavelet technically real-valued
        return wavelet_value;
    }

    /// Building the convolution filter in frequency space (real-valued to save space)
    pub fn build_freq_filter(&self, fft_size: usize, scale: f64) -> Vec<f64> {
        // Initialize the filter
        let mut filter: Vec<f64> = vec![0.0; fft_size];

        // Filling in filter value
        // Note analyticity means negative frequencies are zero
        let df = 2.0 * std::f64::consts::PI / fft_size as f64;
        for k in 0..=fft_size/2 {
            let omega = df * k as f64;
            filter[k] = self.freq_coefficient_value(omega, scale);
        }

        // Return the real-valued filter
        return filter;
    }
}
