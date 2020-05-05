use crate::math::{ field, polynom, parallel };
use crate::stark::{ MAX_CONSTRAINT_DEGREE, utils::CompositionCoefficients };

// TYPES AND INTERFACES
// ================================================================================================
pub struct ConstraintPoly {
    poly: Vec<u64>
}

// CONSTRAINT POLY IMPLEMENTATION
// ================================================================================================
impl ConstraintPoly {

    pub fn new(poly: Vec<u64>) -> ConstraintPoly {

        assert!(poly.len().is_power_of_two(), "poly length must be a power of two");
        debug_assert!(get_expected_degree(&poly) == polynom::degree_of(&poly),
            "expected polynomial of degree {} but received degree {}",
            get_expected_degree(&poly),
            polynom::degree_of(&poly));

        return ConstraintPoly { poly };
    }

    pub fn degree(&self) -> usize {
        return get_expected_degree(&self.poly);
    }

    pub fn eval(&self, twiddles: &[u64]) -> Vec<u64> {
        let domain_size = twiddles.len() * 2;
        assert!(domain_size > self.poly.len(), "domain size must be greater than poly length");

        let mut evaluations = vec![0; domain_size];
        evaluations[..self.poly.len()].copy_from_slice(&self.poly);
        polynom::eval_fft_twiddles(&mut evaluations, twiddles, true);

        return evaluations;
    }

    pub fn eval_at(&self, z: u64) -> u64 {
        return polynom::eval(&self.poly, z);
    }

    pub fn merge_into(mut self, result: &mut Vec<u64>, z: u64, cc: &CompositionCoefficients) -> u64 {

        // evaluate the polynomial at point z
        let z_value = polynom::eval(&self.poly, z);

        // compute C(x) = (P(x) - P(z)) / (x - z)
        self.poly[0] = field::sub(self.poly[0], z_value);
        polynom::syn_div_in_place(&mut self.poly, z);

        // add C(x) * cc into the result
        parallel::mul_acc(result, &self.poly, cc.constraints, 1);

        return z_value;
    }

}

// HELPER FUNCTIONS
// ================================================================================================
fn get_expected_degree(poly: &[u64]) -> usize {
    let trace_length = poly.len() / MAX_CONSTRAINT_DEGREE;
    return poly.len() - trace_length;
}