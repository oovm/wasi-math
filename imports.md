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
<h4><a name="syntax_error"><code>enum syntax-error</code></a></h4>
<p>Parse failed</p>
<h5>Enum Cases</h5>
<ul>
<li>
<p><a name="syntax_error.empty"><code>empty</code></a></p>
<p>The input string is empty
</li>
<li>
<p><a name="syntax_error.invalid"><code>invalid</code></a></p>
<p>The input string is not a valid number
</li>
</ul>
<h4><a name="math_error"><code>variant math-error</code></a></h4>
<h5>Variant Cases</h5>
<ul>
<li>
<p><a name="math_error.syntax"><code>syntax</code></a>: <a href="#syntax_error"><a href="#syntax_error"><code>syntax-error</code></a></a></p>
<p>The operation is not supported
</li>
<li>
<p><a name="math_error.divide_zero"><code>divide-zero</code></a></p>
<p>Trying to divide zero
</li>
</ul>
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
<h4><a name="integer"><code>resource integer</code></a></h4>
<p>The basic unit of big numbers, which can be shared in big-integer, big-fraction, big-complex and other composite structures.</p>
<p>Unsigned integers are immutable shared objects. Therefore, some memory allocation can be avoided when creating operations such as inverse numbers, absolute values, fractions, etc.</p>
<p>Equal big integers do not necessarily have the same resource ID. As long as every bit that makes up the large integer is equal, the two large integers are equal.</p>
<h4><a name="integer_buffer"><code>resource integer-buffer</code></a></h4>
<h4><a name="fraction"><code>resource fraction</code></a></h4>
<p>The numerator and denominator must be the simplest fraction, that is, gcd(numerator, denominator) = 1.</p>
<h2>Zero</h2>
<p>The canonical form is fraction { sign = no-sign, numerator = 0, denominator = 1 }.</p>
<h2>If the sign is no-sign, then the number is considered to represent zero and the numerator and denominator are no longer checked.</h2>
<h3>Functions</h3>
<h4><a name="static_integer.from_u32"><code>[static]integer.from-u32: func</code></a></h4>
<h5>Params</h5>
<ul>
<li><a name="static_integer.from_u32.value"><code>value</code></a>: <code>u32</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="static_integer.from_u32.0"></a> own&lt;<a href="#integer"><a href="#integer"><code>integer</code></a></a>&gt;</li>
</ul>
<h4><a name="static_integer.from_u64"><code>[static]integer.from-u64: func</code></a></h4>
<h5>Params</h5>
<ul>
<li><a name="static_integer.from_u64.value"><code>value</code></a>: <code>u64</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="static_integer.from_u64.0"></a> own&lt;<a href="#integer"><a href="#integer"><code>integer</code></a></a>&gt;</li>
</ul>
<h4><a name="static_integer.parse"><code>[static]integer.parse: func</code></a></h4>
<h5>Params</h5>
<ul>
<li><a name="static_integer.parse.text"><code>text</code></a>: <code>string</code></li>
<li><a name="static_integer.parse.radix"><code>radix</code></a>: <code>u8</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="static_integer.parse.0"></a> result&lt;own&lt;<a href="#integer"><a href="#integer"><code>integer</code></a></a>&gt;, <a href="#math_error"><a href="#math_error"><code>math-error</code></a></a>&gt;</li>
</ul>
<h4><a name="method_integer.add"><code>[method]integer.add: func</code></a></h4>
<h5>Params</h5>
<ul>
<li><a name="method_integer.add.self"><code>self</code></a>: borrow&lt;<a href="#integer"><a href="#integer"><code>integer</code></a></a>&gt;</li>
<li><a name="method_integer.add.other"><code>other</code></a>: borrow&lt;<a href="#integer"><a href="#integer"><code>integer</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_integer.add.0"></a> own&lt;<a href="#integer"><a href="#integer"><code>integer</code></a></a>&gt;</li>
</ul>
<h4><a name="method_integer.sub"><code>[method]integer.sub: func</code></a></h4>
<h5>Params</h5>
<ul>
<li><a name="method_integer.sub.self"><code>self</code></a>: borrow&lt;<a href="#integer"><a href="#integer"><code>integer</code></a></a>&gt;</li>
<li><a name="method_integer.sub.other"><code>other</code></a>: borrow&lt;<a href="#integer"><a href="#integer"><code>integer</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_integer.sub.0"></a> own&lt;<a href="#integer"><a href="#integer"><code>integer</code></a></a>&gt;</li>
</ul>
<h4><a name="method_integer.mul"><code>[method]integer.mul: func</code></a></h4>
<h5>Params</h5>
<ul>
<li><a name="method_integer.mul.self"><code>self</code></a>: borrow&lt;<a href="#integer"><a href="#integer"><code>integer</code></a></a>&gt;</li>
<li><a name="method_integer.mul.other"><code>other</code></a>: borrow&lt;<a href="#integer"><a href="#integer"><code>integer</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_integer.mul.0"></a> own&lt;<a href="#integer"><a href="#integer"><code>integer</code></a></a>&gt;</li>
</ul>
<h4><a name="method_integer.div"><code>[method]integer.div: func</code></a></h4>
<h5>Params</h5>
<ul>
<li><a name="method_integer.div.self"><code>self</code></a>: borrow&lt;<a href="#integer"><a href="#integer"><code>integer</code></a></a>&gt;</li>
<li><a name="method_integer.div.other"><code>other</code></a>: borrow&lt;<a href="#integer"><a href="#integer"><code>integer</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_integer.div.0"></a> result&lt;own&lt;<a href="#integer"><a href="#integer"><code>integer</code></a></a>&gt;, <a href="#math_error"><a href="#math_error"><code>math-error</code></a></a>&gt;</li>
</ul>
<h4><a name="method_integer.as_f32"><code>[method]integer.as-f32: func</code></a></h4>
<h5>Params</h5>
<ul>
<li><a name="method_integer.as_f32.self"><code>self</code></a>: borrow&lt;<a href="#integer"><a href="#integer"><code>integer</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_integer.as_f32.0"></a> <code>float32</code></li>
</ul>
<h4><a name="method_integer.as_u32"><code>[method]integer.as-u32: func</code></a></h4>
<h5>Params</h5>
<ul>
<li><a name="method_integer.as_u32.self"><code>self</code></a>: borrow&lt;<a href="#integer"><a href="#integer"><code>integer</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_integer.as_u32.0"></a> result&lt;<code>u32</code>, <a href="#math_error"><a href="#math_error"><code>math-error</code></a></a>&gt;</li>
</ul>
<h4><a name="method_integer.as_f64"><code>[method]integer.as-f64: func</code></a></h4>
<h5>Params</h5>
<ul>
<li><a name="method_integer.as_f64.self"><code>self</code></a>: borrow&lt;<a href="#integer"><a href="#integer"><code>integer</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_integer.as_f64.0"></a> <code>float64</code></li>
</ul>
<h4><a name="method_integer.as_u64"><code>[method]integer.as-u64: func</code></a></h4>
<h5>Params</h5>
<ul>
<li><a name="method_integer.as_u64.self"><code>self</code></a>: borrow&lt;<a href="#integer"><a href="#integer"><code>integer</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_integer.as_u64.0"></a> result&lt;<code>u64</code>, <a href="#math_error"><a href="#math_error"><code>math-error</code></a></a>&gt;</li>
</ul>
<h4><a name="method_integer.clone"><code>[method]integer.clone: func</code></a></h4>
<h5>Params</h5>
<ul>
<li><a name="method_integer.clone.self"><code>self</code></a>: borrow&lt;<a href="#integer"><a href="#integer"><code>integer</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_integer.clone.0"></a> own&lt;<a href="#integer"><a href="#integer"><code>integer</code></a></a>&gt;</li>
</ul>
<h4><a name="method_integer.to_buffer"><code>[method]integer.to-buffer: func</code></a></h4>
<h5>Params</h5>
<ul>
<li><a name="method_integer.to_buffer.self"><code>self</code></a>: borrow&lt;<a href="#integer"><a href="#integer"><code>integer</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_integer.to_buffer.0"></a> own&lt;<a href="#integer_buffer"><a href="#integer_buffer"><code>integer-buffer</code></a></a>&gt;</li>
</ul>
<h4><a name="method_integer.to_radix_string"><code>[method]integer.to-radix-string: func</code></a></h4>
<h5>Params</h5>
<ul>
<li><a name="method_integer.to_radix_string.self"><code>self</code></a>: borrow&lt;<a href="#integer"><a href="#integer"><code>integer</code></a></a>&gt;</li>
<li><a name="method_integer.to_radix_string.radix"><code>radix</code></a>: <code>u8</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_integer.to_radix_string.0"></a> <code>string</code></li>
</ul>
<h4><a name="constructor_integer_buffer"><code>[constructor]integer-buffer: func</code></a></h4>
<h5>Params</h5>
<ul>
<li><a name="constructor_integer_buffer.capacity"><code>capacity</code></a>: <code>u64</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="constructor_integer_buffer.0"></a> own&lt;<a href="#integer_buffer"><a href="#integer_buffer"><code>integer-buffer</code></a></a>&gt;</li>
</ul>
<h4><a name="method_integer_buffer.add_assign"><code>[method]integer-buffer.add-assign: func</code></a></h4>
<h5>Params</h5>
<ul>
<li><a name="method_integer_buffer.add_assign.self"><code>self</code></a>: borrow&lt;<a href="#integer_buffer"><a href="#integer_buffer"><code>integer-buffer</code></a></a>&gt;</li>
<li><a name="method_integer_buffer.add_assign.other"><code>other</code></a>: own&lt;<a href="#integer"><a href="#integer"><code>integer</code></a></a>&gt;</li>
</ul>
<h4><a name="method_integer_buffer.sub_assign"><code>[method]integer-buffer.sub-assign: func</code></a></h4>
<h5>Params</h5>
<ul>
<li><a name="method_integer_buffer.sub_assign.self"><code>self</code></a>: borrow&lt;<a href="#integer_buffer"><a href="#integer_buffer"><code>integer-buffer</code></a></a>&gt;</li>
<li><a name="method_integer_buffer.sub_assign.other"><code>other</code></a>: own&lt;<a href="#integer"><a href="#integer"><code>integer</code></a></a>&gt;</li>
</ul>
<h4><a name="method_integer_buffer.mul_assign"><code>[method]integer-buffer.mul-assign: func</code></a></h4>
<h5>Params</h5>
<ul>
<li><a name="method_integer_buffer.mul_assign.self"><code>self</code></a>: borrow&lt;<a href="#integer_buffer"><a href="#integer_buffer"><code>integer-buffer</code></a></a>&gt;</li>
<li><a name="method_integer_buffer.mul_assign.other"><code>other</code></a>: own&lt;<a href="#integer"><a href="#integer"><code>integer</code></a></a>&gt;</li>
</ul>
<h4><a name="method_integer_buffer.div_assign"><code>[method]integer-buffer.div-assign: func</code></a></h4>
<h5>Params</h5>
<ul>
<li><a name="method_integer_buffer.div_assign.self"><code>self</code></a>: borrow&lt;<a href="#integer_buffer"><a href="#integer_buffer"><code>integer-buffer</code></a></a>&gt;</li>
<li><a name="method_integer_buffer.div_assign.other"><code>other</code></a>: own&lt;<a href="#integer"><a href="#integer"><code>integer</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_integer_buffer.div_assign.0"></a> result&lt;_, <a href="#math_error"><a href="#math_error"><code>math-error</code></a></a>&gt;</li>
</ul>
<h4><a name="method_integer_buffer.finish"><code>[method]integer-buffer.finish: func</code></a></h4>
<h5>Params</h5>
<ul>
<li><a name="method_integer_buffer.finish.self"><code>self</code></a>: borrow&lt;<a href="#integer_buffer"><a href="#integer_buffer"><code>integer-buffer</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_integer_buffer.finish.0"></a> own&lt;<a href="#integer"><a href="#integer"><code>integer</code></a></a>&gt;</li>
</ul>
<h4><a name="constructor_fraction"><code>[constructor]fraction: func</code></a></h4>
<h5>Params</h5>
<ul>
<li><a name="constructor_fraction.numerator"><code>numerator</code></a>: borrow&lt;<a href="#integer"><a href="#integer"><code>integer</code></a></a>&gt;</li>
<li><a name="constructor_fraction.denominator"><code>denominator</code></a>: borrow&lt;<a href="#integer"><a href="#integer"><code>integer</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="constructor_fraction.0"></a> own&lt;<a href="#fraction"><a href="#fraction"><code>fraction</code></a></a>&gt;</li>
</ul>
<h4><a name="method_fraction.sign"><code>[method]fraction.sign: func</code></a></h4>
<h5>Params</h5>
<ul>
<li><a name="method_fraction.sign.self"><code>self</code></a>: borrow&lt;<a href="#fraction"><a href="#fraction"><code>fraction</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_fraction.sign.0"></a> <a href="#sign"><a href="#sign"><code>sign</code></a></a></li>
</ul>
<h4><a name="method_fraction.numerator"><code>[method]fraction.numerator: func</code></a></h4>
<h5>Params</h5>
<ul>
<li><a name="method_fraction.numerator.self"><code>self</code></a>: borrow&lt;<a href="#fraction"><a href="#fraction"><code>fraction</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_fraction.numerator.0"></a> borrow&lt;<a href="#integer"><a href="#integer"><code>integer</code></a></a>&gt;</li>
</ul>
<h4><a name="method_fraction.denominator"><code>[method]fraction.denominator: func</code></a></h4>
<h5>Params</h5>
<ul>
<li><a name="method_fraction.denominator.self"><code>self</code></a>: borrow&lt;<a href="#fraction"><a href="#fraction"><code>fraction</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a name="method_fraction.denominator.0"></a> borrow&lt;<a href="#integer"><a href="#integer"><code>integer</code></a></a>&gt;</li>
</ul>
<h2><a name="wasi:math_conversion_0.2.1_draft">Import interface wasi:math/conversion@0.2.1-draft</a></h2>
<hr />
<h3>Types</h3>
<h4><a name="integer"><code>type integer</code></a></h4>
<p><a href="#integer"><a href="#integer"><code>integer</code></a></a></p>
<p>
## <a name="wasi:math_arithmetic_0.2.1_draft">Import interface wasi:math/arithmetic@0.2.1-draft</a>
<hr />
<h3>Types</h3>
<h4><a name="integer"><code>type integer</code></a></h4>
<p><a href="#integer"><a href="#integer"><code>integer</code></a></a></p>
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
