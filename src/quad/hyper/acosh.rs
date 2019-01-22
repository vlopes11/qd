// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::quad::Quad;

impl Quad {
    /// Calculates the inverse hyperbolic cosine of the number.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let x = qd!(1.5).acosh();
    /// let expected = qd!("0.9624236501192068949955178268487368462703686687713210393220363377");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-30));
    /// # }
    /// ```
    pub fn acosh(self) -> Quad {
        if self < Quad::ONE {
            Quad::NAN
        } else if self.is_infinite() {
            Quad::INFINITY
        } else {
            (self + (self.sqr() - Quad::ONE).sqrt()).ln()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calc() {
        assert_close!(
            qd!("1.811526272460853107021852049305420510220702081057922474861595623"),
            Quad::PI.acosh()
        );
        assert_close!(
            qd!("1.657454454153077272593828742280534739158392762033676825848582209"),
            Quad::E.acosh()
        );
    }

    #[test]
    fn edge() {
        assert_exact!(Quad::NAN, qd!(0.0).acosh());
        assert_exact!(Quad::NAN, Quad::NAN.acosh());
        assert_exact!(Quad::INFINITY, Quad::INFINITY.acosh());
        assert_exact!(Quad::NAN, Quad::NEG_INFINITY.acosh());
    }
}
