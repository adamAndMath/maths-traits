
use crate::algebra::*;
use crate::analysis::*;

///
///A real-valued function on a set `X` that quantifies the "distance" between objects
///
///This is rigorously defined as a function `d:X⨯X -> R` such that
/// * `d(x,y) > 0` for all `x != y`
/// * `d(x,x) = 0` for all `x`
/// * `d(x,z) <= d(x,y) + d(y,z)` for all `x`,`y`, and `z`
///
/// ## Implementation Note
///
///Due to the fact that there are often _many_ metrics for any given space and that a given
///kind of metric often applies to a whole family of spaces, this trait has been written
///with the intent of being implemented on type _other_ than the set `X` to which it applies.
///This way, a construction can implemented only once while still having access to all applicable
///metrics.
///
///A good example of this would a struct implementing all L<sup>p</sup> norms for a single
///implementation of a given ℝ<sup>3</sup> or similar
///
///
pub trait Metric<X, R:Real> {
    fn distance(&self, x1:X, x2:X) -> R;
}

///
///A real-valued function from a [RingModule] that quantifies it's length, allowing for null-vectors
///
///Specifically, a seminorm ‖•‖ is a function from a ring module `X` over `K` to the reals such that:
/// * `K` has a seminorm |•|
/// * `‖x‖ >= 0`
/// * `‖cx‖ = |c|‖x‖`
/// * `‖x+y‖ <= ‖x‖ + ‖y‖`
///
///This is distinct from a [NormedMetric] in that it is allowed to be 0 for non-zero vectors
///
pub trait Seminorm<K:UnitalRing, X:RingModule<K>, R:Real> {
    #[inline] fn norm(&self, x:X) -> R;
    #[inline] fn normalize(&self, x:X) -> X where K:From<R> {x.clone() * K::from(self.norm(x).inv())}
}

///
///A real-valued on a [RingModule] that quantifies it's length, disallowing null-vectors
///
///Specifically, a norm ‖x‖ is a function from a ring module `X` over `K` to the reals such that:
/// * `K` has a norm |•|
/// * `‖x‖ > 0` for all `x != 0`
/// * `‖cx‖ = |c|‖x‖`
/// * `‖x+y‖ <= ‖x‖ + ‖y‖`
///
///This is distinct from a [Seminorm] in that it is _not_ allowed to be 0 for non-zero vectors
///
pub trait Norm<K:UnitalRing, X:RingModule<K>, R:Real>: Seminorm<K,X,R> {}

pub trait InnerProduct<K:ComplexRing, M:RingModule<K>>: HermitianForm<K,M> + Norm<K,M,K::Real>{}

auto! {
    pub trait NormedMetric<K,X,R> = Norm<K,X,R> + Metric<X,R> where K:UnitalRing, X:RingModule<K>, R:Real;
}

///
///A metric on vector-spaces using the [inner product](InnerProductSpace) of two vectors
///
///For finite dimensional real-vector spaces, this is simply the Euclidean metric, and for functions
///on measure-spaces, this gives the L2-metric
///
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct InnerProductMetric;

impl<R:Real, V:InnerProductSpace<R>> Metric<V,R> for InnerProductMetric {
    #[inline(always)] fn distance(&self, x1:V, x2:V) -> R {x1.dist_euclid(x2)}
}

impl<K:ComplexRing, V:InnerProductSpace<K>> Seminorm<K,V,K::Real> for InnerProductMetric {
    #[inline(always)] fn norm(&self, x:V) -> K::Real {x.norm()}
}

impl<K:ComplexRing, V:InnerProductSpace<K>> Norm<K,V,K::Real> for InnerProductMetric {}

impl<K:ComplexRing, V:InnerProductSpace<K>> SesquilinearForm<K,V> for InnerProductMetric {
    #[inline] fn product_of(&self, v1:V, v2:V) -> K {v1.inner_product(v2)}
    #[inline] fn sigma(&self, x:K) -> K {x.conj()}
    #[inline] fn sigma_inv(&self, x:K) -> K {x.conj()}
}

impl<K:ComplexRing, V:InnerProductSpace<K>> ReflexiveForm<K,V> for InnerProductMetric {}
impl<K:ComplexRing, V:InnerProductSpace<K>> SymSesquilinearForm<K,V> for InnerProductMetric {}
impl<K:Real, V:InnerProductSpace<K>> BilinearForm<K,V> for InnerProductMetric {}


pub trait InnerProductSpace<F: ComplexRing>: RingModule<F> {
    fn inner_product(self, rhs:Self) -> F;

    #[inline] fn norm_sqrd(self) -> F::Real {self.clone().inner_product(self).as_real()}
    #[inline] fn norm(self) -> F::Real {self.norm_sqrd().sqrt()}
    #[inline] fn dist_euclid(self, rhs: Self) -> F::Real {(self - rhs).norm()}
    #[inline] fn normalized(self) -> Self where F:From<<F as ComplexSubset>::Real> {
        self.clone() * F::from(self.norm().inv())
    }

    #[inline] fn orthogonal(self, rhs:Self) -> bool {self.inner_product(rhs).is_zero()}
    #[inline] fn reject(self, rhs: Self) -> Self where F:ComplexField { rhs.clone() - self.project(rhs) }
    #[inline] fn project(self, rhs: Self) -> Self where F:ComplexField {
        let l = self.clone().inner_product(self.clone()).inv() * self.clone().inner_product(rhs);
        self * l
    }

    #[inline] fn angle(self, rhs: Self) -> F where F:Trig+From<<F as ComplexSubset>::Real> {
        let l1 = self.clone().norm();
        let l2 = rhs.clone().norm();
        (self.inner_product(rhs) * (l1*l2).inv().into()).acos()
    }
}

macro_rules! impl_metric {
    (@int $($f:ident)*) => {$(
        impl InnerProductSpace<$f> for $f {
            #[inline(always)] fn inner_product(self, rhs:Self) -> $f {self * rhs}
            #[inline(always)] fn norm(self) -> <$f as ComplexSubset>::Real {$f::as_real(self.abs())}
            #[inline(always)] fn orthogonal(self, rhs:Self) -> bool {self==0 || rhs==0}
        }
    )*};
    (@float $($f:ident)*) => {$(
        impl InnerProductSpace<$f> for $f {

            #[inline(always)] fn inner_product(self, rhs:Self) -> $f {self * rhs}
            #[inline(always)] fn norm(self) -> $f {self.abs()}
            #[inline(always)] fn normalized(self) -> $f {self.signum()}

            #[inline(always)] fn orthogonal(self, rhs:Self) -> bool {self==0.0 || rhs==0.0}
            #[inline(always)] fn reject(self, rhs: Self) -> Self { if self==0.0 {rhs} else {0.0} }
            #[inline(always)] fn project(self, rhs: Self) -> Self { if self==0.0 {0.0} else {rhs} }

            #[inline(always)]
            fn angle(self, rhs: Self) -> $f {
                if (self==0.0) ^ (rhs==0.0) {
                    ::core::$f::consts::FRAC_PI_2
                } else if (self<0.0) ^ (rhs<0.0) {
                    ::core::$f::consts::PI
                } else {
                    0.0
                }
            }
        }
    )*}
}
impl_metric!(@float f32 f64);
impl_metric!(@int i32 i64);
