use crate::interface::MultilinearPolynomialInterface;
use crate::multilinear::Multilinear;
use ark_ff::PrimeField;

/// This is a composition of multilinear polynomials whose binding operation is multiplication
pub struct ComposedMultilinear<F: PrimeField> {
    /// These are all the multilinear polynomials
    pub polys: Vec<Multilinear<F>>,
}

impl<F: PrimeField> ComposedMultilinear<F> {
    /// This is the constructor for the composed multilinear polynomial
    pub fn new(polys: Vec<Multilinear<F>>) -> Self {
        // check to see that all the polynomials have the same number of variables
        let n_vars = polys[0].num_vars();
        assert!(polys.iter().all(|p| p.num_vars() == n_vars));

        ComposedMultilinear { polys }
    }
}

impl<F: PrimeField> MultilinearPolynomialInterface<F> for ComposedMultilinear<F> {
    fn num_vars(&self) -> usize {
        self.polys[0].num_vars()
    }

    fn partial_evaluation(&self, evaluation_point: F, variable_index: usize) -> Self {
        todo!()
    }

    fn partial_evaluations(&self, evaluation_points: Vec<F>, variable_indices: Vec<usize>) -> Self {
        todo!()
    }

    fn evaluate(&self, point: &Vec<F>) -> Option<F> {
        let mut result = F::one();

        for poly in &self.polys {
            let eval = poly.evaluate(point);
            match eval {
                Some(val) => result *= val,
                None => return None,
            }
        }

        Some(result)
    }

    fn extend_with_new_variables(&self, num_of_new_variables: usize) -> Self {
        todo!()
    }

    fn add_distinct(&self, rhs: &Self) -> Self {
        todo!()
    }

    fn mul_distinct(&self, rhs: &Self) -> Self {
        todo!()
    }

    fn interpolate(y_s: &[F]) -> Self {
        todo!()
    }

    fn zero(num_vars: usize) -> Self {
        todo!()
    }

    fn is_zero(&self) -> bool {
        todo!()
    }

    fn internal_add(&self, rhs: &Self) -> Self {
        todo!()
    }

    fn internal_add_assign(&mut self, rhs: &Self) {
        todo!()
    }

    fn to_bytes(&self) -> Vec<u8> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_test_curves::bls12_381::Fr;

    #[test]
    fn test_evaluation() {
        let poly1 = Multilinear::new(vec![Fr::from(0), Fr::from(1), Fr::from(2), Fr::from(3)], 2);
        let poly2 = Multilinear::new(vec![Fr::from(0), Fr::from(0), Fr::from(0), Fr::from(1)], 2);

        let composed = ComposedMultilinear::new(vec![poly1, poly2]);

        let eval = composed.evaluate(&vec![Fr::from(2), Fr::from(3)]);
        assert_eq!(eval, Some(Fr::from(42)));
    }
}