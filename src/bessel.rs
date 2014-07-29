/*
 * A rust binding for the GSL library by Guillaume Gomez (guillaume1.gomez@gmail.com)
 */

pub mod Bessel {
    use ffi;
    use types::*;
    use std::mem::zeroed;

    /// These routines compute the regular modified cylindrical Bessel function of zeroth order, I_0(x)
    pub fn I0(x: f64) -> f64 {
        unsafe { ffi::gsl_sf_bessel_I0(x) }
    }

    pub fn I0_e(x: f64) -> (i32, GslResult) {
        let mut result = unsafe { zeroed::<ffi::gsl_sf_result>() };
        let ret = unsafe { ffi::gsl_sf_bessel_I0_e(x, &mut result) };

        (ret, GslResult{val: result.val, err: result.err})
    }

    /// These routines compute the regular modified cylindrical Bessel function of first order, I_1(x).
    pub fn I1(x: f64) -> f64 {
        unsafe { ffi::gsl_sf_bessel_I1(x) }
    }

    pub fn I1_e(x: f64) -> (i32, GslResult) {
        let mut result = unsafe { zeroed::<ffi::gsl_sf_result>() };
        let ret = unsafe { ffi::gsl_sf_bessel_I1_e(x, &mut result) };

        (ret, GslResult{val: result.val, err: result.err})
    }

    /// These routines compute the regular modified cylindrical Bessel function of order n, I_n(x).
    pub fn In(n: i32, x: f64) -> f64 {
        unsafe { ffi::gsl_sf_bessel_In(n, x) }
    }

    pub fn In_e(n: i32, x: f64) -> (i32, GslResult) {
        let mut result = unsafe { zeroed::<ffi::gsl_sf_result>() };
        let ret = unsafe { ffi::gsl_sf_bessel_In_e(n, x, &mut result) };

        (ret, GslResult{val: result.val, err: result.err})
    }

    /// This routine computes the values of the regular modified cylindrical Bessel functions I_n(x) for n from nmin to nmax inclusive, storing the results in the array result_array.
    /// The start of the range nmin must be positive or zero.
    /// The values are computed using recurrence relations for efficiency, and therefore may differ slightly from the exact values.
    pub fn In_array(nmin: i32, nmax: i32, x: f64, result_array: &mut [f64]) -> i32 {
        unsafe { ffi::gsl_sf_bessel_In_array(nmin, nmax, x, result_array.as_mut_ptr()) }
    }

    /// These routines compute the scaled regular modified cylindrical Bessel function of zeroth order \exp(-|x|) I_0(x).
    pub fn I0_scaled(x: f64) -> f64 {
        unsafe { ffi::gsl_sf_bessel_I0_scaled(x) }
    }

    pub fn I0_scaled_e(x: f64) -> (i32, GslResult) {
        let mut result = unsafe { zeroed::<ffi::gsl_sf_result>() };
        let ret = unsafe { ffi::gsl_sf_bessel_I0_scaled_e(x, &mut result) };

        (ret, GslResult{val: result.val, err: result.err})
    }

    /// These routines compute the scaled regular modified cylindrical Bessel function of first order \exp(-|x|) I_1(x).
    pub fn I1_scaled(x: f64) -> f64 {
        unsafe { ffi::gsl_sf_bessel_I1_scaled(x) }
    }

    pub fn I1_scaled_e(x: f64) -> (i32, GslResult) {
        let mut result = unsafe { zeroed::<ffi::gsl_sf_result>() };
        let ret = unsafe { ffi::gsl_sf_bessel_I1_scaled_e(x, &mut result) };

        (ret, GslResult{val: result.val, err: result.err})
    }

    /// These routines compute the scaled regular modified cylindrical Bessel function of order n, \exp(-|x|) I_n(x)
    pub fn In_scaled(n: i32, x: f64) -> f64 {
        unsafe { ffi::gsl_sf_bessel_In_scaled(n, x) }
    }

    pub fn In_scaled_e(n: i32, x: f64) -> (i32, GslResult) {
        let mut result = unsafe { zeroed::<ffi::gsl_sf_result>() };
        let ret = unsafe { ffi::gsl_sf_bessel_In_scaled_e(n, x, &mut result) };

        (ret, GslResult{val: result.val, err: result.err})
    }

    /// This routine computes the values of the scaled regular cylindrical Bessel functions \exp(-|x|) I_n(x) for n from nmin to nmax inclusive, storing the results in the array result_array.
    /// The start of the range nmin must be positive or zero.
    /// The values are computed using recurrence relations for efficiency, and therefore may differ slightly from the exact values.
    pub fn In_scaled_array(nmin: i32, nmax: i32, x: f64, result_array: &mut [f64]) -> i32 {
        unsafe { ffi::gsl_sf_bessel_In_scaled_array(nmin, nmax, x, result_array.as_mut_ptr()) }
    }

    /// These routines compute the scaled regular modified spherical Bessel function of zeroth order, \exp(-|x|) i_0(x).
    pub fn i0_scaled(x: f64) -> f64 {
        unsafe { ffi::gsl_sf_bessel_i0_scaled(x) }
    }

    pub fn i0_scaled_e(x: f64) -> (i32, GslResult) {
        let mut result = unsafe { zeroed::<ffi::gsl_sf_result>() };
        let ret = unsafe { ffi::gsl_sf_bessel_i0_scaled_e(x, &mut result) };

        (ret, GslResult{val: result.val, err: result.err})
    }

    /// These routines compute the scaled regular modified spherical Bessel function of first order, \exp(-|x|) i_1(x).
    pub fn i1_scaled(x: f64) -> f64 {
        unsafe { ffi::gsl_sf_bessel_i1_scaled(x) }
    }

    pub fn i1_scaled_e(x: f64) -> (i32, GslResult) {
        let mut result = unsafe { zeroed::<ffi::gsl_sf_result>() };
        let ret = unsafe { ffi::gsl_sf_bessel_i1_scaled_e(x, &mut result) };

        (ret, GslResult{val: result.val, err: result.err})
    }

    /// These routines compute the scaled regular modified spherical Bessel function of second order, \exp(-|x|) i_2(x)
    pub fn i2_scaled(x: f64) -> f64 {
        unsafe { ffi::gsl_sf_bessel_i2_scaled(x) }
    }

    pub fn i2_scaled_e(x: f64) -> (i32, GslResult) {
        let mut result = unsafe { zeroed::<ffi::gsl_sf_result>() };
        let ret = unsafe { ffi::gsl_sf_bessel_i2_scaled_e(x, &mut result) };

        (ret, GslResult{val: result.val, err: result.err})
    }

    /// These routines compute the scaled regular modified spherical Bessel function of order l, \exp(-|x|) i_l(x)
    pub fn il_scaled(l: i32, x: f64) -> f64 {
        unsafe { ffi::gsl_sf_bessel_il_scaled(l, x) }
    }

    pub fn il_scaled_e(l: i32, x: f64) -> (i32, GslResult) {
        let mut result = unsafe { zeroed::<ffi::gsl_sf_result>() };
        let ret = unsafe { ffi::gsl_sf_bessel_il_scaled_e(l, x, &mut result) };

        (ret, GslResult{val: result.val, err: result.err})
    }

    /// This routine computes the values of the scaled regular modified cylindrical Bessel functions \exp(-|x|) i_l(x) for l from 0 to lmax inclusive for lmax >= 0, storing the results in the array result_array. The values are computed using recurrence relations for efficiency, and therefore may differ slightly from the exact values.
    pub fn il_scaled_array(lmax: i32, x: f64, result_array: &mut [f64]) -> i32 {
        unsafe { ffi::gsl_sf_bessel_il_scaled_array(lmax, x, result_array.as_mut_ptr()) }
    }

    /// These routines compute the regular modified Bessel function of fractional order \nu, I_\nu(x) for x>0, \nu>0.
    pub fn Inu(nu: f64, x: f64) -> f64 {
        unsafe { ffi::gsl_sf_bessel_Inu(nu, x) }
    }

    pub fn Inu_e(nu: f64, x: f64) -> (i32, GslResult) {
        let mut result = unsafe { zeroed::<ffi::gsl_sf_result>() };
        let ret = unsafe { ffi::gsl_sf_bessel_Inu_e(nu, x, &mut result) };

        (ret, GslResult{val: result.val, err: result.err})
    }

    /// These routines compute the scaled regular modified Bessel function of fractional order \nu, \exp(-|x|)I_\nu(x) for x>0, \nu>0.
    pub fn Inu_scaled(nu: f64, x: f64) -> f64 {
        unsafe { ffi::gsl_sf_bessel_Inu_scaled(nu, x) }
    }

    pub fn Inu_scaled_e(nu: f64, x: f64) -> (i32, GslResult) {
        let mut result = unsafe { zeroed::<ffi::gsl_sf_result>() };
        let ret = unsafe { ffi::gsl_sf_bessel_Inu_scaled_e(nu, x, &mut result) };

        (ret, GslResult{val: result.val, err: result.err})
    }

    /// These routines compute the regular cylindrical Bessel function of zeroth order, J_0(x).
    pub fn J0(x: f64) -> f64 {
        unsafe { ffi::gsl_sf_bessel_J0(x) }
    }

    pub fn J0_e(x: f64) -> (i32, GslResult) {
        let mut result = unsafe { zeroed::<ffi::gsl_sf_result>() };
        let ret = unsafe { ffi::gsl_sf_bessel_J0_e(x, &mut result) };

        (ret, GslResult{val: result.val, err: result.err})
    }

    /// These routines compute the regular cylindrical Bessel function of first order, J_1(x).
    pub fn J1(x: f64) -> f64 {
        unsafe { ffi::gsl_sf_bessel_J1(x) }
    }

    pub fn J1_e(x: f64) -> (i32, GslResult) {
        let mut result = unsafe { zeroed::<ffi::gsl_sf_result>() };
        let ret = unsafe { ffi::gsl_sf_bessel_J1_e(x, &mut result) };

        (ret, GslResult{val: result.val, err: result.err})
    }

    /// These routines compute the regular cylindrical Bessel function of order n, J_n(x).
    pub fn Jn(n: i32, x: f64) -> f64 {
        unsafe { ffi::gsl_sf_bessel_Jn(n, x) }
    }

    pub fn Jn_e(n: i32, x: f64) -> (i32, GslResult) {
        let mut result = unsafe { zeroed::<ffi::gsl_sf_result>() };
        let ret = unsafe { ffi::gsl_sf_bessel_Jn_e(n, x, &mut result) };

        (ret, GslResult{val: result.val, err: result.err})
    }

    /// This routine computes the values of the regular cylindrical Bessel functions J_n(x) for n from nmin to nmax inclusive, storing the results in the array result_array.
    /// The values are computed using recurrence relations for efficiency, and therefore may differ slightly from the exact values.
    pub fn Jn_array(nmin: i32, nmax: i32, x: f64, result_array: &mut [f64]) -> i32 {
        unsafe { ffi::gsl_sf_bessel_Jn_array(nmin, nmax, x, result_array.as_mut_ptr()) }
    }

    /// These routines compute the regular spherical Bessel function of zeroth order, j_0(x) = \sin(x)/x.
    pub fn j0(x: f64) -> f64 {
        unsafe { ffi::gsl_sf_bessel_j0(x) }
    }

    pub fn j0_e(x: f64) -> (i32, GslResult) {
        let mut result = unsafe { zeroed::<ffi::gsl_sf_result>() };
        let ret = unsafe { ffi::gsl_sf_bessel_j0_e(x, &mut result) };

        (ret, GslResult{val: result.val, err: result.err})
    }

    /// These routines compute the regular spherical Bessel function of first order, j_1(x) = (\sin(x)/x - \cos(x))/x.
    pub fn j1(x: f64) -> f64 {
        unsafe { ffi::gsl_sf_bessel_j1(x) }
    }

    pub fn j1_e(x: f64) -> (i32, GslResult) {
        let mut result = unsafe { zeroed::<ffi::gsl_sf_result>() };
        let ret = unsafe { ffi::gsl_sf_bessel_j1_e(x, &mut result) };

        (ret, GslResult{val: result.val, err: result.err})
    }

    /// These routines compute the regular spherical Bessel function of second order, j_2(x) = ((3/x^2 - 1)\sin(x) - 3\cos(x)/x)/x.
    pub fn j2(x: f64) -> f64 {
        unsafe { ffi::gsl_sf_bessel_j2(x) }
    }

    pub fn j2_e(x: f64) -> (i32, GslResult) {
        let mut result = unsafe { zeroed::<ffi::gsl_sf_result>() };
        let ret = unsafe { ffi::gsl_sf_bessel_j2_e(x, &mut result) };

        (ret, GslResult{val: result.val, err: result.err})
    }

    /// These routines compute the regular spherical Bessel function of order l, j_l(x), for l >= 0 and x >= 0.
    pub fn jl(l: i32, x: f64) -> f64 {
        unsafe { ffi::gsl_sf_bessel_jl(l, x) }
    }

    pub fn jl_e(l: i32, x: f64) -> (i32, GslResult) {
        let mut result = unsafe { zeroed::<ffi::gsl_sf_result>() };
        let ret = unsafe { ffi::gsl_sf_bessel_jl_e(l, x, &mut result) };

        (ret, GslResult{val: result.val, err: result.err})
    }

    /// This routine computes the values of the regular spherical Bessel functions j_l(x) for l from 0 to lmax inclusive for lmax >= 0 and x >= 0, storing the results in the array result_array.
    /// The values are computed using recurrence relations for efficiency, and therefore may differ slightly from the exact values.
    pub fn jl_array(lmax: i32, x: f64, result_array: &mut [f64]) -> i32 {
        unsafe { ffi::gsl_sf_bessel_jl_array(lmax, x, result_array.as_mut_ptr()) }
    }

    /// This routine uses Steed’s method to compute the values of the regular spherical Bessel functions j_l(x) for l from 0 to lmax inclusive for lmax >= 0 and x >= 0, storing the results in the array result_array.
    /// The Steed/Barnett algorithm is described in Comp. Phys. Comm. 21, 297 (1981). Steed’s method is more stable than the recurrence used in the other functions but is also slower.
    pub fn jl_steed_array(lmax: i32, x: f64, result_array: &mut [f64]) -> i32 {
        unsafe { ffi::gsl_sf_bessel_jl_steed_array(lmax, x, result_array.as_mut_ptr()) }
    }

    /// These routines compute the regular cylindrical Bessel function of fractional order \nu, J_\nu(x).
    pub fn Jnu(nu: f64, x: f64) -> f64 {
        unsafe { ffi::gsl_sf_bessel_Jnu(nu, x) }
    }

    pub fn Jnu_e(nu: f64, x: f64) -> (i32, GslResult) {
        let mut result = unsafe { zeroed::<ffi::gsl_sf_result>() };
        let ret = unsafe { ffi::gsl_sf_bessel_Jnu_e(nu, x, &mut result) };

        (ret, GslResult{val: result.val, err: result.err})
    }

    /// This function computes the regular cylindrical Bessel function of fractional order \nu, J_\nu(x), evaluated at a series of x values. The array v of length size contains the x values.
    /// They are assumed to be strictly ordered and positive. The array is over-written with the values of J_\nu(x_i).
    pub fn sequence_Jnu(nu: f64, mode: Gsl::Mode, v: &mut [f64]) -> i32 {
        unsafe { ffi::gsl_sf_bessel_sequence_Jnu_e(nu, mode as u32, v.len() as i64, v.as_mut_ptr()) }
    }
}