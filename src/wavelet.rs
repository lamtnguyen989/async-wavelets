use num_complex::{Complex64};

/***
*   Generalized Morse Wavelet
***/

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
}

/// Normalization constant metric context
pub enum Normalization{
    L1, // Convention to use this
    L2
};

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
    pub fn new(param: GMW_Params, norm_type: Normalization) -> Self {
        return Self {
            params: params,
            alpha: calculate_normalization_const(&params, norm_type)
        }
    }

    pub fn new(beta: f64, gamma: f64, norm_type: Normalization) -> Self {
        params = GMW_Params::new(beta, gamma);
        return Self {
            params: params,
            alpha: calculate_normalization_const(&params, norm_type)
        }
    }

    /// Calculate normalization constant
    fn calculate_normalization_const(&param: GMW_Params, norm_type: Normalization) -> f64 {
        match norm_type {
            Normalization::L1   => {todo!();},
            Normalization::L2   => {todo!();},
        }
    }
}
