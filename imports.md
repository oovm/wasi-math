<h1><a name="imports">World imports</a></h1>
<ul>
<li>Imports:
<ul>
<li>interface <a href="#wasi:math_types_0.2.1_draft"><code>wasi:math/types@0.2.1-draft</code></a></li>
<li>interface <a href="#wasi:math_conversion_0.2.1_draft"><code>wasi:math/conversion@0.2.1-draft</code></a></li>
<li>interface <a href="#wasi:math_arithmetic_0.2.1_draft"><code>wasi:math/arithmetic@0.2.1-draft</code></a></li>
</ul>
</li>
</ul>
<h2><a name="wasi:math_types_0.2.1_draft">Import interface wasi:math/types@0.2.1-draft</a></h2>
<hr />
<h3>Types</h3>
<h4><a name="unsigned_integer"><code>resource unsigned-integer</code></a></h4>
<p>The basic unit of big numbers, which can be shared in big-integer, big-fraction, big-complex and other composite structures.</p>
<h4><a name="sign"><code>enum sign</code></a></h4>
<h5>Enum Cases</h5>
<ul>
<li>
<p><a name="sign.no_sign"><code>no-sign</code></a></p>
<p>Neither positive nor negative, such as zero
</li>
<li>
<p><a name="sign.positive"><code>positive</code></a></p>
<p>This is a positive number greater than zero
</li>
<li>
<p><a name="sign.negative"><code>negative</code></a></p>
<p>This is a negative number less than zero
</li>
</ul>
<h4><a name="integer"><code>record integer</code></a></h4>
<h5>Record Fields</h5>
<ul>
<li><a name="integer.sign"><a href="#sign"><code>sign</code></a></a>: <a href="#sign"><a href="#sign"><code>sign</code></a></a></li>
<li><a name="integer.digits"><code>digits</code></a>: own&lt;<a href="#unsigned_integer"><a href="#unsigned_integer"><code>unsigned-integer</code></a></a>&gt;</li>
</ul>
<h4><a name="fraction"><code>record fraction</code></a></h4>
<p>The numerator and denominator must be the simplest fraction, that is, gcd(numerator, denominator) = 1.</p>
<h2>Zero</h2>
<p>The canonical form is fraction { sign = no-sign, numerator = 0, denominator = 1 }.</p>
<p>If the sign is no-sign, then the number is considered to represent zero and the numerator and denominator are no longer checked.</p>
<h5>Record Fields</h5>
<ul>
<li><a name="fraction.sign"><a href="#sign"><code>sign</code></a></a>: <a href="#sign"><a href="#sign"><code>sign</code></a></a></li>
<li><a name="fraction.numerator"><code>numerator</code></a>: own&lt;<a href="#unsigned_integer"><a href="#unsigned_integer"><code>unsigned-integer</code></a></a>&gt;</li>
<li><a name="fraction.denominator"><code>denominator</code></a>: own&lt;<a href="#unsigned_integer"><a href="#unsigned_integer"><code>unsigned-integer</code></a></a>&gt;</li>
</ul>
<hr />
<h3>Functions</h3>
<h4><a name="static_unsigned_integer.from_u32"><code>[static]unsigned-integer.from-u32: func</code></a></h4>
<h5>Params</h5>
<ul>
<li><a name="static_unsigned_integer.from_u32.value"><code>value</code></a>: <code>u32</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="static_unsigned_integer.from_u32.0"></a> own&lt;<a href="#unsigned_integer"><a href="#unsigned_integer"><code>unsigned-integer</code></a></a>&gt;</li>
</ul>
<h4><a name="static_unsigned_integer.from_u64"><code>[static]unsigned-integer.from-u64: func</code></a></h4>
<h5>Params</h5>
<ul>
<li><a name="static_unsigned_integer.from_u64.value"><code>value</code></a>: <code>u64</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="static_unsigned_integer.from_u64.0"></a> own&lt;<a href="#unsigned_integer"><a href="#unsigned_integer"><code>unsigned-integer</code></a></a>&gt;</li>
</ul>
<h4><a name="method_unsigned_integer.add"><code>[method]unsigned-integer.add: func</code></a></h4>
<h5>Params</h5>
<ul>
<li><a name="method_unsigned_integer.add.self"><code>self</code></a>: borrow&lt;<a href="#unsigned_integer"><a href="#unsigned_integer"><code>unsigned-integer</code></a></a>&gt;</li>
<li><a name="method_unsigned_integer.add.other"><code>other</code></a>: own&lt;<a href="#unsigned_integer"><a href="#unsigned_integer"><code>unsigned-integer</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_unsigned_integer.add.0"></a> own&lt;<a href="#unsigned_integer"><a href="#unsigned_integer"><code>unsigned-integer</code></a></a>&gt;</li>
</ul>
<h4><a name="method_unsigned_integer.sub"><code>[method]unsigned-integer.sub: func</code></a></h4>
<h5>Params</h5>
<ul>
<li><a name="method_unsigned_integer.sub.self"><code>self</code></a>: borrow&lt;<a href="#unsigned_integer"><a href="#unsigned_integer"><code>unsigned-integer</code></a></a>&gt;</li>
<li><a name="method_unsigned_integer.sub.other"><code>other</code></a>: own&lt;<a href="#unsigned_integer"><a href="#unsigned_integer"><code>unsigned-integer</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_unsigned_integer.sub.0"></a> own&lt;<a href="#unsigned_integer"><a href="#unsigned_integer"><code>unsigned-integer</code></a></a>&gt;</li>
</ul>
<h4><a name="method_unsigned_integer.sub_saturating"><code>[method]unsigned-integer.sub-saturating: func</code></a></h4>
<h5>Params</h5>
<ul>
<li><a name="method_unsigned_integer.sub_saturating.self"><code>self</code></a>: borrow&lt;<a href="#unsigned_integer"><a href="#unsigned_integer"><code>unsigned-integer</code></a></a>&gt;</li>
<li><a name="method_unsigned_integer.sub_saturating.other"><code>other</code></a>: own&lt;<a href="#unsigned_integer"><a href="#unsigned_integer"><code>unsigned-integer</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_unsigned_integer.sub_saturating.0"></a> own&lt;<a href="#unsigned_integer"><a href="#unsigned_integer"><code>unsigned-integer</code></a></a>&gt;</li>
</ul>
<h4><a name="method_unsigned_integer.sub_checked"><code>[method]unsigned-integer.sub-checked: func</code></a></h4>
<h5>Params</h5>
<ul>
<li><a name="method_unsigned_integer.sub_checked.self"><code>self</code></a>: borrow&lt;<a href="#unsigned_integer"><a href="#unsigned_integer"><code>unsigned-integer</code></a></a>&gt;</li>
<li><a name="method_unsigned_integer.sub_checked.other"><code>other</code></a>: own&lt;<a href="#unsigned_integer"><a href="#unsigned_integer"><code>unsigned-integer</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_unsigned_integer.sub_checked.0"></a> option&lt;own&lt;<a href="#unsigned_integer"><a href="#unsigned_integer"><code>unsigned-integer</code></a></a>&gt;&gt;</li>
</ul>
<h4><a name="method_unsigned_integer.mul"><code>[method]unsigned-integer.mul: func</code></a></h4>
<h5>Params</h5>
<ul>
<li><a name="method_unsigned_integer.mul.self"><code>self</code></a>: borrow&lt;<a href="#unsigned_integer"><a href="#unsigned_integer"><code>unsigned-integer</code></a></a>&gt;</li>
<li><a name="method_unsigned_integer.mul.other"><code>other</code></a>: own&lt;<a href="#unsigned_integer"><a href="#unsigned_integer"><code>unsigned-integer</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_unsigned_integer.mul.0"></a> own&lt;<a href="#unsigned_integer"><a href="#unsigned_integer"><code>unsigned-integer</code></a></a>&gt;</li>
</ul>
<h4><a name="method_unsigned_integer.div"><code>[method]unsigned-integer.div: func</code></a></h4>
<h5>Params</h5>
<ul>
<li><a name="method_unsigned_integer.div.self"><code>self</code></a>: borrow&lt;<a href="#unsigned_integer"><a href="#unsigned_integer"><code>unsigned-integer</code></a></a>&gt;</li>
<li><a name="method_unsigned_integer.div.other"><code>other</code></a>: own&lt;<a href="#unsigned_integer"><a href="#unsigned_integer"><code>unsigned-integer</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_unsigned_integer.div.0"></a> own&lt;<a href="#unsigned_integer"><a href="#unsigned_integer"><code>unsigned-integer</code></a></a>&gt;</li>
</ul>
<h4><a name="method_unsigned_integer.as_f32"><code>[method]unsigned-integer.as-f32: func</code></a></h4>
<h5>Params</h5>
<ul>
<li><a name="method_unsigned_integer.as_f32.self"><code>self</code></a>: borrow&lt;<a href="#unsigned_integer"><a href="#unsigned_integer"><code>unsigned-integer</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_unsigned_integer.as_f32.0"></a> <code>float32</code></li>
</ul>
<h4><a name="method_unsigned_integer.as_f64"><code>[method]unsigned-integer.as-f64: func</code></a></h4>
<h5>Params</h5>
<ul>
<li><a name="method_unsigned_integer.as_f64.self"><code>self</code></a>: borrow&lt;<a href="#unsigned_integer"><a href="#unsigned_integer"><code>unsigned-integer</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_unsigned_integer.as_f64.0"></a> <code>float64</code></li>
</ul>
<h2><a name="wasi:math_conversion_0.2.1_draft">Import interface wasi:math/conversion@0.2.1-draft</a></h2>
<hr />
<h3>Types</h3>
<h4><a name="unsigned_integer"><code>type unsigned-integer</code></a></h4>
<p><a href="#unsigned_integer"><a href="#unsigned_integer"><code>unsigned-integer</code></a></a></p>
<p>
#### <a name="integer">`type integer`</a>
[`integer`](#integer)
<p>
## <a name="wasi:math_arithmetic_0.2.1_draft">Import interface wasi:math/arithmetic@0.2.1-draft</a>
<hr />
<h3>Types</h3>
<h4><a name="unsigned_integer"><code>type unsigned-integer</code></a></h4>
<p><a href="#unsigned_integer"><a href="#unsigned_integer"><code>unsigned-integer</code></a></a></p>
<p>
#### <a name="integer">`type integer`</a>
[`integer`](#integer)
<p>
----
<h3>Functions</h3>
<h4><a name="pow_f32"><code>pow-f32: func</code></a></h4>
<p>Raises a number to a floating point power.</p>
<h5>Params</h5>
<ul>
<li><a name="pow_f32.base"><code>base</code></a>: <code>float32</code></li>
<li><a name="pow_f32.exponent"><code>exponent</code></a>: <code>float32</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="pow_f32.0"></a> <code>float32</code></li>
</ul>
<h4><a name="pow_f64"><code>pow-f64: func</code></a></h4>
<p>Raises a number to a floating point power.</p>
<h5>Params</h5>
<ul>
<li><a name="pow_f64.base"><code>base</code></a>: <code>float64</code></li>
<li><a name="pow_f64.exponent"><code>exponent</code></a>: <code>float64</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="pow_f64.0"></a> <code>float64</code></li>
</ul>
<h4><a name="cbrt_f32"><code>cbrt-f32: func</code></a></h4>
<p>Returns the cube root of a number, more accurate than using pow function</p>
<h5>Params</h5>
<ul>
<li><a name="cbrt_f32.value"><code>value</code></a>: <code>float32</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="cbrt_f32.0"></a> <code>float32</code></li>
</ul>
<h4><a name="cbrt_f64"><code>cbrt-f64: func</code></a></h4>
<p>Returns the cube root of a number, more accurate than using pow function</p>
<h5>Params</h5>
<ul>
<li><a name="cbrt_f64.value"><code>value</code></a>: <code>float64</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="cbrt_f64.0"></a> <code>float64</code></li>
</ul>
<h4><a name="hypot_f32"><code>hypot-f32: func</code></a></h4>
<p>Compute the distance between the origin and a point (x, y) on the Euclidean plane. Equivalently, compute the length of the hypotenuse of a right-angle triangle with other sides having length x.abs() and y.abs().</p>
<h5>Params</h5>
<ul>
<li><a name="hypot_f32.x"><code>x</code></a>: <code>float32</code></li>
<li><a name="hypot_f32.y"><code>y</code></a>: <code>float32</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="hypot_f32.0"></a> <code>float32</code></li>
</ul>
<h4><a name="hypot_f64"><code>hypot-f64: func</code></a></h4>
<p>Compute the distance between the origin and a point (x, y) on the Euclidean plane. Equivalently, compute the length of the hypotenuse of a right-angle triangle with other sides having length x.abs() and y.abs().</p>
<h5>Params</h5>
<ul>
<li><a name="hypot_f64.x"><code>x</code></a>: <code>float32</code></li>
<li><a name="hypot_f64.y"><code>y</code></a>: <code>float32</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="hypot_f64.0"></a> <code>float32</code></li>
</ul>
<h4><a name="exp_f32"><code>exp-f32: func</code></a></h4>
<p>Returns the exponential e^value, more accurate than using pow function</p>
<h5>Params</h5>
<ul>
<li><a name="exp_f32.value"><code>value</code></a>: <code>float32</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="exp_f32.0"></a> <code>float32</code></li>
</ul>
<h4><a name="exp_f64"><code>exp-f64: func</code></a></h4>
<p>Returns the exponential e^value, more accurate than using pow function</p>
<h5>Params</h5>
<ul>
<li><a name="exp_f64.value"><code>value</code></a>: <code>float64</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="exp_f64.0"></a> <code>float64</code></li>
</ul>
<h4><a name="exp_m1_f32"><code>exp-m1-f32: func</code></a></h4>
<p>Returns e^value - 1 in a way that is accurate even if the number is close to zero.</p>
<h5>Params</h5>
<ul>
<li><a name="exp_m1_f32.value"><code>value</code></a>: <code>float32</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="exp_m1_f32.0"></a> <code>float32</code></li>
</ul>
<h4><a name="exp_m1_f64"><code>exp-m1-f64: func</code></a></h4>
<p>Returns e^value - 1 in a way that is accurate even if the number is close to zero.</p>
<h5>Params</h5>
<ul>
<li><a name="exp_m1_f64.value"><code>value</code></a>: <code>float64</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="exp_m1_f64.0"></a> <code>float64</code></li>
</ul>
<h4><a name="log_f32"><code>log-f32: func</code></a></h4>
<p>Returns the logarithm of the number with respect to an arbitrary base.</p>
<h5>Params</h5>
<ul>
<li><a name="log_f32.base"><code>base</code></a>: <code>float32</code></li>
<li><a name="log_f32.value"><code>value</code></a>: <code>float32</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="log_f32.0"></a> <code>float32</code></li>
</ul>
<h4><a name="log_f64"><code>log-f64: func</code></a></h4>
<p>Returns the logarithm of the number with respect to an arbitrary base.</p>
<h5>Params</h5>
<ul>
<li><a name="log_f64.base"><code>base</code></a>: <code>float64</code></li>
<li><a name="log_f64.value"><code>value</code></a>: <code>float64</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="log_f64.0"></a> <code>float64</code></li>
</ul>
<h4><a name="ln_f32"><code>ln-f32: func</code></a></h4>
<p>Returns the natural logarithm of the number, more accurate than using log function</p>
<h5>Params</h5>
<ul>
<li><a name="ln_f32.value"><code>value</code></a>: <code>float32</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="ln_f32.0"></a> <code>float32</code></li>
</ul>
<h4><a name="ln_f64"><code>ln-f64: func</code></a></h4>
<p>Returns the natural logarithm of the number, more accurate than using log function</p>
<h5>Params</h5>
<ul>
<li><a name="ln_f64.value"><code>value</code></a>: <code>float64</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="ln_f64.0"></a> <code>float64</code></li>
</ul>
<h4><a name="ln_p1_f32"><code>ln-p1-f32: func</code></a></h4>
<p>Returns ln(n+1) (natural logarithm) more accurately than if the operations were performed separately.</p>
<h5>Params</h5>
<ul>
<li><a name="ln_p1_f32.value"><code>value</code></a>: <code>float32</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="ln_p1_f32.0"></a> <code>float32</code></li>
</ul>
<h4><a name="ln_p1_f64"><code>ln-p1-f64: func</code></a></h4>
<p>Returns ln(n+1) (natural logarithm) more accurately than if the operations were performed separately.</p>
<h5>Params</h5>
<ul>
<li><a name="ln_p1_f64.value"><code>value</code></a>: <code>float64</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="ln_p1_f64.0"></a> <code>float64</code></li>
</ul>
<h4><a name="log2_f32"><code>log2-f32: func</code></a></h4>
<p>Returns the base 2 logarithm of the number, more accurate than using log function</p>
<h5>Params</h5>
<ul>
<li><a name="log2_f32.value"><code>value</code></a>: <code>float32</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="log2_f32.0"></a> <code>float32</code></li>
</ul>
<h4><a name="log2_f64"><code>log2-f64: func</code></a></h4>
<p>Returns the base 2 logarithm of the number, more accurate than using log function</p>
<h5>Params</h5>
<ul>
<li><a name="log2_f64.value"><code>value</code></a>: <code>float64</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="log2_f64.0"></a> <code>float64</code></li>
</ul>
<h4><a name="log10_f32"><code>log10-f32: func</code></a></h4>
<p>Returns the base 10 logarithm of the number, more accurate than using log function</p>
<h5>Params</h5>
<ul>
<li><a name="log10_f32.value"><code>value</code></a>: <code>float32</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="log10_f32.0"></a> <code>float32</code></li>
</ul>
<h4><a name="log10_f64"><code>log10-f64: func</code></a></h4>
<p>Returns the base 10 logarithm of the number, more accurate than using log function</p>
<h5>Params</h5>
<ul>
<li><a name="log10_f64.value"><code>value</code></a>: <code>float64</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="log10_f64.0"></a> <code>float64</code></li>
</ul>
<h4><a name="cos_f32"><code>cos-f32: func</code></a></h4>
<p>Computes the cosine of a number (in radians).</p>
<h5>Params</h5>
<ul>
<li><a name="cos_f32.value"><code>value</code></a>: <code>float32</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="cos_f32.0"></a> <code>float32</code></li>
</ul>
<h4><a name="cos_f64"><code>cos-f64: func</code></a></h4>
<p>Computes the cosine of a number (in radians).</p>
<h5>Params</h5>
<ul>
<li><a name="cos_f64.value"><code>value</code></a>: <code>float64</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="cos_f64.0"></a> <code>float64</code></li>
</ul>
<h4><a name="cosh_f32"><code>cosh-f32: func</code></a></h4>
<p>Computes the hyperbolic cosine of a number (in radians).</p>
<h5>Params</h5>
<ul>
<li><a name="cosh_f32.value"><code>value</code></a>: <code>float32</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="cosh_f32.0"></a> <code>float32</code></li>
</ul>
<h4><a name="cosh_f64"><code>cosh-f64: func</code></a></h4>
<p>Computes the hyperbolic cosine of a number (in radians).</p>
<h5>Params</h5>
<ul>
<li><a name="cosh_f64.value"><code>value</code></a>: <code>float64</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="cosh_f64.0"></a> <code>float64</code></li>
</ul>
<h4><a name="sin_f32"><code>sin-f32: func</code></a></h4>
<p>Computes the sine of a number (in radians).</p>
<h5>Params</h5>
<ul>
<li><a name="sin_f32.value"><code>value</code></a>: <code>float32</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="sin_f32.0"></a> <code>float32</code></li>
</ul>
<h4><a name="sin_f64"><code>sin-f64: func</code></a></h4>
<p>Computes the sine of a number (in radians).</p>
<h5>Params</h5>
<ul>
<li><a name="sin_f64.value"><code>value</code></a>: <code>float64</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="sin_f64.0"></a> <code>float64</code></li>
</ul>
<h4><a name="sinh_f32"><code>sinh-f32: func</code></a></h4>
<p>Computes the hyperbolic of a number (in radians).</p>
<h5>Params</h5>
<ul>
<li><a name="sinh_f32.value"><code>value</code></a>: <code>float32</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="sinh_f32.0"></a> <code>float32</code></li>
</ul>
<h4><a name="sinh_f64"><code>sinh-f64: func</code></a></h4>
<p>Computes the hyperbolic of a number (in radians).</p>
<h5>Params</h5>
<ul>
<li><a name="sinh_f64.value"><code>value</code></a>: <code>float64</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="sinh_f64.0"></a> <code>float64</code></li>
</ul>
<h4><a name="tan_f32"><code>tan-f32: func</code></a></h4>
<p>Computes the tangent of a number (in radians).</p>
<h5>Params</h5>
<ul>
<li><a name="tan_f32.value"><code>value</code></a>: <code>float32</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="tan_f32.0"></a> <code>float32</code></li>
</ul>
<h4><a name="tan_f64"><code>tan-f64: func</code></a></h4>
<p>Computes the tangent of a number (in radians).</p>
<h5>Params</h5>
<ul>
<li><a name="tan_f64.value"><code>value</code></a>: <code>float64</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="tan_f64.0"></a> <code>float64</code></li>
</ul>
<h4><a name="tanh_f32"><code>tanh-f32: func</code></a></h4>
<p>Computes the hyperbolic tangent of a number (in radians).</p>
<h5>Params</h5>
<ul>
<li><a name="tanh_f32.value"><code>value</code></a>: <code>float32</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="tanh_f32.0"></a> <code>float32</code></li>
</ul>
<h4><a name="tanh_f64"><code>tanh-f64: func</code></a></h4>
<p>Computes the hyperbolic tangent of a number (in radians).</p>
<h5>Params</h5>
<ul>
<li><a name="tanh_f64.value"><code>value</code></a>: <code>float64</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="tanh_f64.0"></a> <code>float64</code></li>
</ul>
<h4><a name="acos_f32"><code>acos-f32: func</code></a></h4>
<p>Computes the arccosine of a number.</p>
<p>Return value is in radians in the range [0, π] or NaN if the number is outside the range [-1, 1].</p>
<h5>Params</h5>
<ul>
<li><a name="acos_f32.value"><code>value</code></a>: <code>float32</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="acos_f32.0"></a> <code>float32</code></li>
</ul>
<h4><a name="acos_f64"><code>acos-f64: func</code></a></h4>
<p>Computes the arccosine of a number.</p>
<p>Return value is in radians in the range [0, π] or NaN if the number is outside the range [-1, 1].</p>
<h5>Params</h5>
<ul>
<li><a name="acos_f64.value"><code>value</code></a>: <code>float64</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="acos_f64.0"></a> <code>float64</code></li>
</ul>
<h4><a name="acosh_f32"><code>acosh-f32: func</code></a></h4>
<p>Returns the inverse hyperbolic cosine of a number</p>
<h5>Params</h5>
<ul>
<li><a name="acosh_f32.value"><code>value</code></a>: <code>float32</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="acosh_f32.0"></a> <code>float32</code></li>
</ul>
<h4><a name="acosh_f64"><code>acosh-f64: func</code></a></h4>
<p>Returns the inverse hyperbolic cosine of a number</p>
<h5>Params</h5>
<ul>
<li><a name="acosh_f64.value"><code>value</code></a>: <code>float64</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="acosh_f64.0"></a> <code>float64</code></li>
</ul>
<h4><a name="asin_f32"><code>asin-f32: func</code></a></h4>
<p>Computes the arcsine of a number.</p>
<p>Return value is in radians in the range [-π/2, π/2] or NaN if the number is outside the range [-1, 1].</p>
<h5>Params</h5>
<ul>
<li><a name="asin_f32.value"><code>value</code></a>: <code>float32</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="asin_f32.0"></a> <code>float32</code></li>
</ul>
<h4><a name="asin_f64"><code>asin-f64: func</code></a></h4>
<p>Computes the arcsine of a number.</p>
<p>Return value is in radians in the range [-π/2, π/2] or NaN if the number is outside the range [-1, 1].</p>
<h5>Params</h5>
<ul>
<li><a name="asin_f64.value"><code>value</code></a>: <code>float64</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="asin_f64.0"></a> <code>float64</code></li>
</ul>
<h4><a name="asinh_f32"><code>asinh-f32: func</code></a></h4>
<p>Returns the inverse hyperbolic sine of a number</p>
<h5>Params</h5>
<ul>
<li><a name="asinh_f32.value"><code>value</code></a>: <code>float32</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="asinh_f32.0"></a> <code>float32</code></li>
</ul>
<h4><a name="asinh_f64"><code>asinh-f64: func</code></a></h4>
<p>Returns the inverse hyperbolic sine of a number</p>
<h5>Params</h5>
<ul>
<li><a name="asinh_f64.value"><code>value</code></a>: <code>float64</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="asinh_f64.0"></a> <code>float64</code></li>
</ul>
<h4><a name="atan_f32"><code>atan-f32: func</code></a></h4>
<p>Computes the arctangent of a number.</p>
<p>Return value is in radians in the range [-π/2, π/2];</p>
<h5>Params</h5>
<ul>
<li><a name="atan_f32.value"><code>value</code></a>: <code>float32</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="atan_f32.0"></a> <code>float32</code></li>
</ul>
<h4><a name="atan_f64"><code>atan-f64: func</code></a></h4>
<p>Computes the arctangent of a number.</p>
<p>Return value is in radians in the range [-π/2, π/2];</p>
<h5>Params</h5>
<ul>
<li><a name="atan_f64.value"><code>value</code></a>: <code>float64</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="atan_f64.0"></a> <code>float64</code></li>
</ul>
<h4><a name="atanh_f32"><code>atanh-f32: func</code></a></h4>
<p>Returns the inverse hyperbolic tangent of a number</p>
<h5>Params</h5>
<ul>
<li><a name="atanh_f32.value"><code>value</code></a>: <code>float32</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="atanh_f32.0"></a> <code>float32</code></li>
</ul>
<h4><a name="atanh_f64"><code>atanh-f64: func</code></a></h4>
<p>Returns the inverse hyperbolic tangent of a number</p>
<h5>Params</h5>
<ul>
<li><a name="atanh_f64.value"><code>value</code></a>: <code>float64</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="atanh_f64.0"></a> <code>float64</code></li>
</ul>
