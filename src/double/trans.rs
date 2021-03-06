// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::double::common::{INV_FACTS, mul_pwr2};
use crate::double::Double;

// #region Exponential

impl Double {
    /// Computes the exponential function, *e*<sup>x</sup>, for the number.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let x = dd!(2).exp();
    /// let expected = dd!("7.3890560989306502272304274605750");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-30));
    /// # }
    /// ```
    pub fn exp(self) -> Double {
        // Strategy, as gleaned from MIT papers and Wikipedia:
        //
        // The first step is to reduce the size of the exponent by noting that
        //
        //      exp(kr + m * ln(2)) = 2^m * exp(r)^k
        //
        // where m and k are arbitary integers. By choosing m appropriately we can make |kr| <=
        // ln(2) / 2 = 0.347. Then exp(r) is evaluated using a Taylor series, which is actually
        // reasonably easy to figure out for the exponential function:
        //
        //      exp(x) = 1 + x + x^2/2! + x^3/3! + x^4/4! ...
        //
        // Reducing x substantially speeds up the convergence, so we have to use fewer terms to
        // reach the required precision.

        let k = 512.0;
        let inv_k = 1.0 / k;

        // Common cases, including numbers too big or small to be represented with Doubles
        if self.0 <= -709.0 {
            return Double::ZERO;
        }
        if self.0 >= 709.0 {
            return Double::INFINITY;
        }
        if self.is_zero() {
            return Double::ONE;
        }
        if self == Double::ONE {
            return Double::E;
        }

        let m = (self.0 / Double::LN_2.0 + 0.5).floor();
        let r = mul_pwr2(self - Double::LN_2 * Double::from(m), inv_k);

        let mut p = r.sqr();
        let mut s = r + mul_pwr2(p, 0.5);
        p *= r;
        let mut t = p * INV_FACTS[0];
        let mut i = 0;

        loop {
            s += t;
            p *= r;
            i += 1;
            t = p * INV_FACTS[i];
            if i >= 5 || t.abs() <= Double::from(inv_k) * Double::EPSILON {
                break;
            }
        }

        s += t;

        s = mul_pwr2(s, 2.0) + s.sqr();
        s = mul_pwr2(s, 2.0) + s.sqr();
        s = mul_pwr2(s, 2.0) + s.sqr();
        s = mul_pwr2(s, 2.0) + s.sqr();
        s = mul_pwr2(s, 2.0) + s.sqr();
        s = mul_pwr2(s, 2.0) + s.sqr();
        s = mul_pwr2(s, 2.0) + s.sqr();
        s = mul_pwr2(s, 2.0) + s.sqr();
        s = mul_pwr2(s, 2.0) + s.sqr();
        s += Double::ONE;

        s.ldexp(m as i32)
    }
}

// #endregion

// #region Logarithms

impl Double {
    /// Calculates the natural logarithm, log<sub>*e*</sub>, of the number.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let x = Double::from(7).ln();
    /// let expected = dd!("1.9459101490553133051053527434432");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-30));
    /// # }
    /// ```
    pub fn ln(self) -> Double {
        // Strategy:
        //
        // The Taylor series for logarithms converges much more slowly than that of exp because of
        // the lack of a factorial term in the denominator. Hence this routine instead tries to
        // determine the root of the function
        //
        //      f(x) = exp(x) - a
        //
        // using Newton's iteration. This iteration is given by
        //
        //      x' = x - f(x)/f'(x)
        //         = x - (1 - a * exp(-x))
        //         = x + a * exp(-x) - 1
        //
        // Testing has shown that it requires two iterations to get the required precision.
        if self == Double::ONE {
            return Double::ZERO;
        }
        if self.is_zero() || self.is_sign_negative() {
            return Double::NAN;
        }

        let mut x = Double::from(self.0.ln()); // initial approximation
        let mut i = 0;
        loop {
            let next = x + self * (-x).exp() - Double::ONE;
            if (x - next).abs() < Double::EPSILON || i >= 5 {
                return next;
            }
            x = next;
            i += 1;
        }
    }

    /// Calculates log<sub>10</sub> of the number.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let x = Double::E.log10();
    /// let expected = dd!("0.434294481903251827651128918916605");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-30));
    /// # }
    /// ```
    #[inline]
    pub fn log10(self) -> Double {
        self.ln() / Double::LN_10
    }

    /// Calculates log<sub>2</sub> of the number.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let x = dd!(10).log2();
    /// let expected = dd!("3.32192809488736234787031942948939");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-30));
    /// # }
    /// ```
    #[inline]
    pub fn log2(self) -> Double {
        self.ln() / Double::LN_2
    }

    /// Calculates the base `b` logarithm (log<sub>`b`</sub>) of the number.
    ///
    /// If the goal is to calculate the base *e*, base 2, or base 10 logarithms of `self`, the
    /// specialized functions for those purposes([`ln`], [`log2`], and [`log10`] respectively) will
    /// be more efficient.
    ///
    /// [`ln`]: #method.ln
    /// [`log2`]: #method.log2
    /// [`log10`]: #method.log10
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let x = dd!(10).log(7.0);
    /// let expected = dd!("1.18329466245493832681792856164686");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-30));
    /// # }
    /// ```
    #[inline]
    pub fn log(self, b: f64) -> Double {
        self.ln() / Double::from(b).ln()
    }
}

// #endregion

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_close {
        ($expected:expr, $actual:expr $(,)*) => {
            assert_precision!($expected, $actual, 30);
        };
    }

    #[test]
    fn exp_whole() {
        assert_close!(dd!("20.085536923187667740928529654582"), dd!(3).exp());
        assert_exact!(Double::INFINITY, dd!("13579135791357913579").exp());
        assert_close!(dd!("20.085536923187667740928529654582"), dd!(3e0).exp());
        assert_close!(dd!("0.049787068367863942979342415650062"), dd!(-3).exp());
    }

    #[test]
    fn exp_repr() {
        assert_close!(dd!("33.115451958692313750653249350389"), dd!(3.5).exp());
        assert_exact!(Double::INFINITY, dd!("13579135791357913579.5").exp());
        assert_close!(dd!("33.115451958692313750653249350389"), dd!(3.5e0).exp());
        assert_close!(dd!("0.030197383422318500739786292363620"), dd!(-3.5).exp());
    }

    #[test]
    fn exp_unrepr() {
        assert_close!(dd!("27.112638920657887426818372110231"), dd!(3.3).exp());
        assert_exact!(Double::INFINITY, dd!("13579135791357913579.3").exp());
        assert_close!(dd!("27.112638920657887426818372110231"), dd!(3.3e0).exp());
        assert_close!(dd!("0.036883167401240005445603704741515"), dd!(-3.3).exp());
    }

    #[test]
    fn exp_edge() {
        // Check for less precision here, as floating point numbers lose precision when this large
        assert_precision!(
            dd!("7.4363225878808657251823671807256e307"),
            dd!(708.9).exp(),
            27
        );
        assert_exact!(Double::INFINITY, dd!(709).exp());
        // Check exact because we actually can't check precision this close to the negative limit
        // of floating point exponents, but precision is only f64 precision anyway
        assert_exact!(
            dd!("3.6554113896149252338842678903601e-308"),
            dd!(-707.9).exp()
        );
        assert_exact!(Double::ZERO, dd!(-709).exp());
        assert_exact!(Double::E, Double::ONE.exp());
        assert_exact!(Double::ONE, Double::ZERO.exp());
    }

    #[test]
    fn ln_whole() {
        assert_close!(dd!("1.0986122886681096913952452369225"), dd!(3).ln());
        assert_close!(
            dd!("44.055066155659480309583232783789"),
            dd!("13579135791357913579").ln()
        );
        assert_close!(dd!("33.334803590584749267647125602504"), dd!(3e14).ln());
    }

    #[test]
    fn ln_repr() {
        assert_close!(dd!("1.2527629684953679956881206219850"), dd!(3.5).ln());
        assert_close!(
            dd!("44.055066155659480309620053978280"),
            dd!("13579135791357913579.5").ln()
        );
        assert_close!(dd!("33.488954270412007571940000987566"), dd!(3.5e14).ln());
    }

    #[test]
    fn ln_unrepr() {
        assert_close!(dd!("1.1939224684724345514391973602033"), dd!(3.3).ln());
        assert_close!(
            dd!("44.055066155659480309605325500484"),
            dd!("13579135791357913579.3").ln()
        );
        assert_close!(dd!("33.430113770389074127691077725784"), dd!(3.3e14).ln());
    }

    #[test]
    fn ln_edge() {
        assert_exact!(Double::ZERO, Double::ONE.ln());
        assert!(Double::ZERO.ln().is_nan());
    }

    #[test]
    fn log10_whole() {
        assert_close!(dd!("0.47712125471966243729502790325512"), dd!(3).log10());
        assert_close!(
            dd!("19.132872131285618236170988986373"),
            dd!("13579135791357913579").log10()
        );
        assert_close!(dd!("14.477121254719662437295027903255"), dd!(3e14).log10());
    }

    #[test]
    fn log10_repr() {
        assert_close!(dd!("0.54406804435027563549847736386814"), dd!(3.5).log10());
        assert_close!(
            dd!("19.132872131285618236186980227958"),
            dd!("13579135791357913579.5").log10()
        );
        assert_close!(
            dd!("14.544068044350275635498477363868"),
            dd!(3.5e14).log10()
        );
    }

    #[test]
    fn log10_unrepr() {
        assert_close!(dd!("0.51851393987788747804522787449814"), dd!(3.3).log10());
        assert_close!(
            dd!("19.132872131285618236180583731324"),
            dd!("13579135791357913579.3").log10()
        );
        assert_close!(
            dd!("14.518513939877887478045227874498"),
            dd!(3.3e14).log10()
        );
    }

    #[test]
    fn log2_whole() {
        assert_close!(dd!("1.5849625007211561814537389439478"), dd!(3).log2());
        assert_close!(
            dd!("63.558025468805141892815343984787"),
            dd!("13579135791357913579").log2()
        );
        assert_close!(dd!("48.091955829144229051638210956799"), dd!(3e14).log2());
    }

    #[test]
    fn log2_repr() {
        assert_close!(dd!("1.8073549220576041074419693172318"), dd!(3.5).log2());
        assert_close!(
            dd!("63.558025468805141892868465739480"),
            dd!("13579135791357913579.5").log2()
        );
        assert_close!(dd!("48.314348250480676977626441330083"), dd!(3.5e14).log2());
    }

    #[test]
    fn log2_unrepr() {
        assert_close!(dd!("1.7224660244710910897827825611842"), dd!(3.3).log2());
        assert_close!(
            dd!("63.558025468805141892847217037603"),
            dd!("13579135791357913579.3").log2()
        );
        assert_close!(dd!("48.229459352894163959967254574036"), dd!(3.3e14).log2());
    }

    #[test]
    fn log_whole() {
        assert_close!(dd!("0.61314719276545841312975386153218"), dd!(3).log(6.0));
        assert_close!(
            dd!("24.587600574891760229061986690548"),
            dd!("13579135791357913579").log(6.0)
        );
        assert_close!(dd!("18.604508117904021052338461213292"), dd!(3e14).log(6.0));
    }

    #[test]
    fn log_repr() {
        assert_close!(dd!("0.66928171360986244557771423441173"), dd!(3.5).log(6.5));
        assert_close!(
            dd!("23.536176364846509016600040652890"),
            dd!("13579135791357913579.5").log(6.5)
        );
        assert_close!(
            dd!("17.891289305927900856466621928281"),
            dd!(3.5e14).log(6.5)
        );
    }

    #[test]
    fn log_unrepr() {
        assert_close!(dd!("0.64867713796370837633691947605569"), dd!(3.3).log(6.3));
        assert_close!(
            dd!("23.935820776719745554239970452887"),
            dd!("13579135791357913579.3").log(6.3)
        );
        assert_close!(
            dd!("18.163114519590611961621954529941"),
            dd!(3.3e14).log(6.3)
        );
    }
}
