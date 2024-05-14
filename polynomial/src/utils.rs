use ark_ff::Field;

use crate::{interface::UnivariantPolynomialInterface, univariant::UnivariantPolynomial};

pub fn get_langrange_basis<F: Field>(domain: &Vec<F>, y_s: &Vec<F>) -> Vec<UnivariantPolynomial<F>> {
    let mut basis = Vec::new();

    for i in 0..domain.len() {
        let mut basis_element = UnivariantPolynomial::default();

        for j in 0..domain.len() {
            if i == j {
                continue;
            }

            // basis_element *= "x - domain[j]" / (domain[i] - domain[j]);
            let numerator = UnivariantPolynomial::from_coefficients_vec(vec![-domain[j], F::one()]);
            let mut  denominator = domain[i] - domain[j];
            denominator.neg_in_place();
            basis_element = numerator * UnivariantPolynomial::from_coefficients_vec(vec![denominator]);
        }

        basis.push(basis_element * UnivariantPolynomial::from_coefficients_vec(vec![y_s[i]]));
    }

    basis
}