use num_complex::{Complex64};

/***
*   Generalized Morse Wavelet
***/

/// Normalization constant metric context
 #[derive(Clone, Copy)]
pub enum Normalization{
    L1, // Convention to use this
    L2
}

/// Parameters deciding which wavelet within the Generalized Morse Wavelet family
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GMW_Params
{
    gamma:  f64,
    beta:   f64,
}

impl GMW_Params
{
    /// Constructor
    pub fn new(beta: f64, gamma: f64) -> Self {
        return Self {
            beta:   beta,
            gamma:  gamma
        }
    }

    /// Getters
    pub fn get_beta(&self) -> f64 {return self.beta;}
    pub fn get_gamma(&self) -> f64 {return self.gamma;}

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

impl From<(f64, f64)> for GMW_Params {
    fn from(tuple: (f64, f64)) -> Self {
        let (beta, gamma) = (tuple.0, tuple.1);
        return GMW_Params::new(beta, gamma);
    }
}

/// Actual Generalized Morse wavelet handler
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GeneralizedMorseWavelet
{
    params: GMW_Params, // Parameter
    alpha:  f64,        // Normalization constant
}

impl GeneralizedMorseWavelet
{
    /// Constructors
    pub fn new<P>(params: P, norm_type: Normalization) -> Self
    where P: Into<GMW_Params>
    {
        let params: GMW_Params = params.into();
        return Self {
            params: params,
            alpha:  params.normalization_const(norm_type)
        }
    }

    /// Wavelet transform coefficient value via convolution in frequency space
    pub fn value(&self, omega: f64, scale: f64) -> Complex64 {
        if omega <= 0.0 {
            return Complex64::new(0.0, 0.0);
        }
        todo!();
    }
}
