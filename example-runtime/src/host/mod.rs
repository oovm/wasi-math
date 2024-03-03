use wasmtime::component::{Resource};
use crate::wasi::math::types::{Fraction, HostFraction, HostInteger, HostIntegerBuffer, HostUnsignedInteger, Integer, IntegerBuffer, MathError, ParseError, Sign, UnsignedInteger};

#[derive(Default)]
pub struct MathContext {}

#[allow(unused_variables)]
impl HostInteger for MathContext {
    fn from_u32(&mut self, value: u32) -> anyhow::Result<Resource<Integer>> {
        todo!()
    }

    fn from_u64(&mut self, value: u64) -> anyhow::Result<Resource<Integer>> {
        todo!()
    }

    fn parse(&mut self, text: String, radix: u32) -> anyhow::Result<Result<Resource<Integer>, ParseError>> {
        todo!()
    }

    fn add(&mut self, self_: Resource<Integer>, other: Resource<Integer>) -> anyhow::Result<Resource<Integer>> {
        todo!()
    }

    fn sub(&mut self, self_: Resource<Integer>, other: Resource<Integer>) -> anyhow::Result<Resource<Integer>> {
        todo!()
    }

    fn mul(&mut self, self_: Resource<Integer>, other: Resource<Integer>) -> anyhow::Result<Resource<Integer>> {
        todo!()
    }

    fn div(&mut self, self_: Resource<Integer>, other: Resource<Integer>) -> anyhow::Result<Resource<Integer>> {
        todo!()
    }

    fn as_f32(&mut self, self_: Resource<Integer>) -> anyhow::Result<f32> {
        todo!()
    }

    fn as_u32(&mut self, self_: Resource<Integer>) -> anyhow::Result<Result<u32, MathError>> {
        todo!()
    }

    fn as_f64(&mut self, self_: Resource<Integer>) -> anyhow::Result<f64> {
        todo!()
    }

    fn as_u64(&mut self, self_: Resource<Integer>) -> anyhow::Result<Result<u64, MathError>> {
        todo!()
    }

    fn clone(&mut self, self_: Resource<Integer>) -> anyhow::Result<Resource<Integer>> {
        todo!()
    }

    fn to_buffer(&mut self, self_: Resource<Integer>) -> anyhow::Result<Resource<IntegerBuffer>> {
        todo!()
    }

    fn to_radix_string(&mut self, self_: Resource<Integer>, radix: u32) -> anyhow::Result<String> {
        todo!()
    }

    fn drop(&mut self, rep: Resource<Integer>) -> anyhow::Result<()> {
        todo!()
    }
}

impl HostIntegerBuffer for MathContext {
    fn new(&mut self, capacity: u64) -> anyhow::Result<Resource<IntegerBuffer>> {
        todo!()
    }

    fn add_assign(&mut self, self_: Resource<IntegerBuffer>, other: Resource<Integer>) -> anyhow::Result<()> {
        todo!()
    }

    fn sub_assign(&mut self, self_: Resource<IntegerBuffer>, other: Resource<Integer>) -> anyhow::Result<()> {
        todo!()
    }

    fn mul_assign(&mut self, self_: Resource<IntegerBuffer>, other: Resource<Integer>) -> anyhow::Result<()> {
        todo!()
    }

    fn div_assign(&mut self, self_: Resource<IntegerBuffer>, other: Resource<Integer>) -> anyhow::Result<Result<(), MathError>> {
        todo!()
    }

    fn finish(&mut self, self_: Resource<IntegerBuffer>) -> anyhow::Result<Resource<Integer>> {
        todo!()
    }

    fn drop(&mut self, rep: Resource<IntegerBuffer>) -> anyhow::Result<()> {
        todo!()
    }
}

impl HostFraction for MathContext {
    fn sign(&mut self, self_: Resource<Fraction>) -> anyhow::Result<Sign> {
        todo!()
    }

    fn numerator(&mut self, self_: Resource<Fraction>) -> anyhow::Result<Resource<Integer>> {
        todo!()
    }

    fn denominator(&mut self, self_: Resource<Fraction>) -> anyhow::Result<Resource<Integer>> {
        todo!()
    }

    fn drop(&mut self, rep: Resource<Fraction>) -> anyhow::Result<()> {
        todo!()
    }
}

impl crate::wasi::math::types::Host for MathContext {}

impl crate::wasi::math::arithmetic::Host for MathContext {
    fn pow_f32(&mut self, base: f32, exponent: f32) -> anyhow::Result<f32> {
         Ok(base.powf(exponent))
    }

    fn pow_f64(&mut self, base: f64, exponent: f64) -> anyhow::Result<f64> {
        Ok(base.powf(exponent))
    }

    fn cbrt_f32(&mut self, value: f32) -> anyhow::Result<f32> {
        Ok(value.cbrt())
    }

    fn cbrt_f64(&mut self, value: f64) -> anyhow::Result<f64> {
        Ok(value.cbrt())
    }

    fn hypot_f32(&mut self, x: f32, y: f32) -> anyhow::Result<f32> {
        Ok(x.hypot(y))
    }

    fn hypot_f64(&mut self, x: f32, y: f32) -> anyhow::Result<f32> {
        Ok(x.hypot(y))
    }

    fn exp_f32(&mut self, value: f32) -> anyhow::Result<f32> {
        Ok(value.exp())
    }

    fn exp_f64(&mut self, value: f64) -> anyhow::Result<f64> {
        Ok(value.exp())
    }

    fn exp_m1_f32(&mut self, value: f32) -> anyhow::Result<f32> {
        Ok(value.exp_m1())
    }

    fn exp_m1_f64(&mut self, value: f64) -> anyhow::Result<f64> {
        Ok(value.exp_m1())
    }

    fn log_f32(&mut self, base: f32, value: f32) -> anyhow::Result<f32> {
        Ok(value.log(base))
    }

    fn log_f64(&mut self, base: f64, value: f64) -> anyhow::Result<f64> {
        Ok(value.log(base))
    }

    fn ln_f32(&mut self, value: f32) -> anyhow::Result<f32> {
        Ok(value.ln())
    }

    fn ln_f64(&mut self, value: f64) -> anyhow::Result<f64> {
        Ok(value.ln())
    }

    fn ln_p1_f32(&mut self, value: f32) -> anyhow::Result<f32> {
        Ok(value.ln_1p())
    }

    fn ln_p1_f64(&mut self, value: f64) -> anyhow::Result<f64> {
        Ok(value.ln_1p())
    }

    fn log2_f32(&mut self, value: f32) -> anyhow::Result<f32> {
        Ok(value.log2())
    }

    fn log2_f64(&mut self, value: f64) -> anyhow::Result<f64> {
        Ok(value.log2())
    }

    fn log10_f32(&mut self, value: f32) -> anyhow::Result<f32> {
        Ok(value.log10())
    }

    fn log10_f64(&mut self, value: f64) -> anyhow::Result<f64> {
        Ok(value.log10())
    }

    fn cos_f32(&mut self, value: f32) -> anyhow::Result<f32> {
        Ok(value.cos())
    }

    fn cos_f64(&mut self, value: f64) -> anyhow::Result<f64> {
        Ok(value.cos())
    }

    fn cosh_f32(&mut self, value: f32) -> anyhow::Result<f32> {
         Ok(value.cosh())
    }

    fn cosh_f64(&mut self, value: f64) -> anyhow::Result<f64> {
        Ok(value.cosh())
    }

    fn sin_f32(&mut self, value: f32) -> anyhow::Result<f32> {
        Ok(value.sin())
    }

    fn sin_f64(&mut self, value: f64) -> anyhow::Result<f64> {
        Ok(value.sin())
    }

    fn sinh_f32(&mut self, value: f32) -> anyhow::Result<f32> {
        Ok(value.sinh())
    }

    fn sinh_f64(&mut self, value: f64) -> anyhow::Result<f64> {
        Ok(value.sinh())
    }

    fn tan_f32(&mut self, value: f32) -> anyhow::Result<f32> {
        Ok(value.tan())
    }

    fn tan_f64(&mut self, value: f64) -> anyhow::Result<f64> {
        Ok(value.tan())
    }

    fn tanh_f32(&mut self, value: f32) -> anyhow::Result<f32> {
        Ok(value.tanh())
    }

    fn tanh_f64(&mut self, value: f64) -> anyhow::Result<f64> {
        Ok(value.tanh())
    }

    fn acos_f32(&mut self, value: f32) -> anyhow::Result<f32> {
        Ok(value.acos())
    }

    fn acos_f64(&mut self, value: f64) -> anyhow::Result<f64> {
        Ok(value.acos())
    }

    fn acosh_f32(&mut self, value: f32) -> anyhow::Result<f32> {
        Ok(value.acosh())
    }

    fn acosh_f64(&mut self, value: f64) -> anyhow::Result<f64> {
        Ok(value.acosh())
    }

    fn asin_f32(&mut self, value: f32) -> anyhow::Result<f32> {
        Ok(value.asin())
    }

    fn asin_f64(&mut self, value: f64) -> anyhow::Result<f64> {
        Ok(value.asin())
    }

    fn asinh_f32(&mut self, value: f32) -> anyhow::Result<f32> {
        Ok(value.asinh())
    }

    fn asinh_f64(&mut self, value: f64) -> anyhow::Result<f64> {
        Ok(value.asinh())
    }

    fn atan_f32(&mut self, value: f32) -> anyhow::Result<f32> {
        Ok(value.atan())
    }

    fn atan_f64(&mut self, value: f64) -> anyhow::Result<f64> {
        Ok(value.atan())
    }

    fn atanh_f32(&mut self, value: f32) -> anyhow::Result<f32> {
        Ok(value.atanh())
    }

    fn atanh_f64(&mut self, value: f64) -> anyhow::Result<f64> {
        Ok(value.atanh())
    }
}

impl crate::wasi::math::conversion::Host for MathContext {}