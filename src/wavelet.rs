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

impl GeneralizedMorseParams
{
    /// Constructor
    pub fn new(beta: f64, gamma: f64) -> Self {
        return Self {
            beta:   beta,
            gamma:  gamma
        }
    }
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
    /// Constructor
    pub fn new(param: GMW_Params, norm_type: Normalization) -> Self {
        todo!();
    }
}
