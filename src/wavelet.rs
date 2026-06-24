use num_complex::{Complex64};

/***
*   Generalized Morse Wavelet
***/

/// Normalization constant metric context
 #[derive(Clone, Copy, Debug)]
pub enum Normalization{
    L1, // Convention to use this
    L2
}

/// Parameters deciding which wavelet within the Generalized Morse Wavelet family
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GmwParams
{
    pub gamma:  f64,
    pub beta:   f64,
}

impl GmwParams
{
    /// Constructor
    pub fn new(beta: f64, gamma: f64) -> Self {
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
                todo!("Calculate L2 normalization constant");
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
    params: GmwParams,  // Parameters
    alpha: f64,         // Normalization constant
    scale_factor: f64,  // Scale factor based on metric context for computing coeffcient values 
}

impl GeneralizedMorseWavelet
{
    /// Constructors
    pub fn new<P>(params: P, norm_type: Normalization) -> Self
    where P: Into<GmwParams>
    {
        let params: GmwParams = params.into();
        let scale_factor = match norm_type {
            Normalization::L1 => 1.0,
            Normalization::L2 => 0.5,
        };

        return Self {
            params:         params,
            alpha:          params.normalization_const(norm_type),
            scale_factor:   scale_factor,
        }
    }

    /// Wavelet transform coefficient value via convolution in frequency space (i.e. a single scalogram value)
    pub fn freq_coefficient_value(&self, omega: f64, scale: f64) -> Complex64 {
        // Heaviside step function in action
        if omega <= 0.0 {
            return Complex64::new(0.0, 0.0);
        }

        // Scale angular frequency
        let s_omega = scale * omega;

        // Calculate unscaled value
        let mut wavelet_value = s_omega.powf(self.params.beta) * f64::exp(-s_omega.powf(self.params.gamma));
        wavelet_value *= self.alpha;    // Normalize the wavelet value

        // Scale the coefficient value based on the metric context
        wavelet_value *= scale.powf(self.scale_factor);

        return Complex64::new(wavelet_value, 0.0);
    }
}
