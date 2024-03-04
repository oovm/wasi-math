use crate::wasi::math::types::{
    Fraction, HostFraction, HostInteger, HostIntegerBuffer, Integer, IntegerBuffer, MathError,
    Sign, SyntaxError,
};
use num::{BigInt, Num, ToPrimitive};
use std::mem::transmute;
use std::ops::{AddAssign, DivAssign, MulAssign, SubAssign};
use wasmtime::component::{Resource, ResourceTable};

#[derive(Default)]
pub struct MathContext {
    pub(crate) table: ResourceTable,
}

#[allow(unused_variables)]
impl HostInteger for MathContext {
    fn from_u32(&mut self, value: u32) -> anyhow::Result<Resource<Integer>> {
        let int = BigInt::from(value);
        Ok(self.table.push(int)?)
    }

    fn from_u64(&mut self, value: u64) -> anyhow::Result<Resource<Integer>> {
        let int = BigInt::from(value);
        Ok(self.table.push(int)?)
    }

    fn parse(
        &mut self,
        text: String,
        radix: u8,
    ) -> anyhow::Result<Result<Resource<Integer>, MathError>> {
        match BigInt::from_str_radix(&text, radix as u32) {
            Ok(int) => Ok(Ok(self.table.push(int)?)),
            Err(e) => Ok(Err(MathError::Syntax(SyntaxError::Invalid))),
        }
    }

    fn add(
        &mut self,
        self_: Resource<Integer>,
        other: Resource<Integer>,
    ) -> anyhow::Result<Resource<Integer>> {
        let lhs = self.table.get(&self_)?;
        let rhs = self.table.get(&other)?;
        Ok(self.table.push(lhs + rhs)?)
    }

    fn sub(
        &mut self,
        self_: Resource<Integer>,
        other: Resource<Integer>,
    ) -> anyhow::Result<Resource<Integer>> {
        let lhs = self.table.get(&self_)?;
        let rhs = self.table.get(&other)?;
        Ok(self.table.push(lhs - rhs)?)
    }

    fn mul(
        &mut self,
        self_: Resource<Integer>,
        other: Resource<Integer>,
    ) -> anyhow::Result<Resource<Integer>> {
        let lhs = self.table.get(&self_)?;
        let rhs = self.table.get(&other)?;
        Ok(self.table.push(lhs * rhs)?)
    }

    fn div(
        &mut self,
        self_: Resource<IntegerBuffer>,
        other: Resource<IntegerBuffer>,
    ) -> anyhow::Result<Result<Resource<IntegerBuffer>, MathError>> {
        let lhs = self.table.get(&self_)?;
        let rhs = self.table.get(&other)?;
        match lhs.checked_div(rhs) {
            Some(s) => Ok(Ok(self.table.push(s)?)),
            None => Ok(Err(MathError::DivideZero)),
        }
    }

    fn as_f32(&mut self, self_: Resource<Integer>) -> anyhow::Result<f32> {
        let lhs = self.table.get(&self_)?;
        // SAFETY: Conversion never fails
        let float = unsafe { lhs.to_f32().unwrap_unchecked() };
        Ok(float)
    }

    fn as_u32(&mut self, self_: Resource<Integer>) -> anyhow::Result<Result<u32, MathError>> {
        todo!()
    }

    fn as_f64(&mut self, self_: Resource<Integer>) -> anyhow::Result<f64> {
        let lhs = self.table.get(&self_)?;
        // SAFETY: Conversion never fails
        let float = unsafe { lhs.to_f64().unwrap_unchecked() };
        Ok(float)
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

    fn to_radix_string(&mut self, self_: Resource<Integer>, radix: u8) -> anyhow::Result<String> {
        todo!()
    }

    fn drop(&mut self, rep: Resource<Integer>) -> anyhow::Result<()> {
        self.table.delete(rep)?;
        Ok(())
    }
}

impl HostIntegerBuffer for MathContext {
    fn new(&mut self, capacity: u64) -> anyhow::Result<Resource<IntegerBuffer>> {
        let int = BigInt::new(Sign::NoSign.into(), Vec::with_capacity(capacity as usize));
        Ok(self.table.push(int)?)
    }

    fn add_assign(
        &mut self,
        self_: Resource<IntegerBuffer>,
        other: Resource<Integer>,
    ) -> anyhow::Result<()> {
        // SAFETY: Double borrowing different values
        let lhs = unsafe {
            let ptr = self.table.get_mut(&self_)?;
            unsafe { transmute::<&mut BigInt, &mut BigInt>(ptr) }
        };
        let rhs = self.table.get(&other)?;
        Ok(lhs.add_assign(rhs))
    }

    fn sub_assign(
        &mut self,
        self_: Resource<IntegerBuffer>,
        other: Resource<Integer>,
    ) -> anyhow::Result<()> {
        // SAFETY: Double borrowing different values
        let lhs = unsafe {
            let ptr = self.table.get_mut(&self_)?;
            unsafe { transmute::<&mut BigInt, &mut BigInt>(ptr) }
        };
        let rhs = self.table.get(&other)?;
        Ok(lhs.sub_assign(rhs))
    }

    fn mul_assign(
        &mut self,
        self_: Resource<IntegerBuffer>,
        other: Resource<Integer>,
    ) -> anyhow::Result<()> {
        // SAFETY: Double borrowing different values
        let lhs = unsafe {
            let ptr = self.table.get_mut(&self_)?;
            unsafe { transmute::<&mut BigInt, &mut BigInt>(ptr) }
        };
        let rhs = self.table.get(&other)?;
        Ok(lhs.mul_assign(rhs))
    }

    fn div_assign(
        &mut self,
        self_: Resource<IntegerBuffer>,
        other: Resource<Integer>,
    ) -> anyhow::Result<Result<(), MathError>> {
        // SAFETY: Double borrowing different values
        let lhs = unsafe {
            let ptr = self.table.get_mut(&self_)?;
            unsafe { transmute::<&mut BigInt, &mut BigInt>(ptr) }
        };
        let rhs = self.table.get(&other)?;
        Ok(Ok(lhs.div_assign(rhs)))
    }

    fn finish(&mut self, self_: Resource<IntegerBuffer>) -> anyhow::Result<Resource<Integer>> {
        Ok(self_)
    }

    fn drop(&mut self, rep: Resource<IntegerBuffer>) -> anyhow::Result<()> {
        self.table.delete(rep)?;
        Ok(())
    }
}

impl HostFraction for MathContext {
    fn new(
        &mut self,
        numerator: Resource<IntegerBuffer>,
        denominator: Resource<IntegerBuffer>,
    ) -> anyhow::Result<Resource<Fraction>> {
        todo!()
    }

    fn sign(&mut self, self_: Resource<Fraction>) -> anyhow::Result<Sign> {
        todo!()
    }

    fn numerator(&mut self, self_: Resource<Fraction>) -> anyhow::Result<Resource<IntegerBuffer>> {
        todo!()
    }

    fn denominator(
        &mut self,
        self_: Resource<Fraction>,
    ) -> anyhow::Result<Resource<IntegerBuffer>> {
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

impl From<Sign> for num::bigint::Sign {
    fn from(value: Sign) -> Self {
        match value {
            Sign::NoSign => num::bigint::Sign::NoSign,
            Sign::Positive => num::bigint::Sign::Plus,
            Sign::Negative => num::bigint::Sign::Minus,
        }
    }
}

impl From<num::bigint::Sign> for Sign {
    fn from(value: num::bigint::Sign) -> Self {
        match value {
            num::bigint::Sign::Minus => Sign::Negative,
            num::bigint::Sign::NoSign => Sign::NoSign,
            num::bigint::Sign::Plus => Sign::Positive,
        }
    }
}
