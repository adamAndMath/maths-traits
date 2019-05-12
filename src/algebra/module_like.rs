
pub use core::ops::{Add, AddAssign, Sub, SubAssign, Neg, Mul, MulAssign, Div, DivAssign, Index, IndexMut};
use algebra::*;

///
///A bi-linear scalar product between [vectors](VectorSpace) or [module](RingModule) elements
///
///Rigorously, a bilinear form is a function `B:X⨯X -> K` from a [RingModule] `X` over `K` into `K` such that:
/// * `B(x+y,z) = B(x,z) + B(y,z)`
/// * `B(x,y+z) = B(x,y) + B(x,z)`
/// * `B(c*x,y) = c*B(x,y)`
/// * `B(x,c*y) = c*B(x,y)`
///
///It is of note, however, that commutivity is not immediately implied by this definition, and to
///assert this is
///
pub trait BilinearForm<K: UnitalRing> { fn dot(self, rhs: Self) -> K; }
pub trait SymmetricForm<K: UnitalRing>: BilinearForm<K> {}
pub trait AlternatingForm<K: UnitalRing>: BilinearForm<K> {}

pub trait QuadradicForm<K: UnitalRing> { fn q_form(self) -> K; }

pub trait ConvergentBasis<K>: Index<usize,Output=K> {fn basis(i:usize) -> Self;}
pub trait CountableBasis<K>: ConvergentBasis<K> + IndexMut<usize, Output=K> {fn elements(&self) -> usize;}
pub trait FiniteBasis<K>: CountableBasis<K> { fn dimensions() -> usize; }

auto!{
    ///An abelian additive group with a distributive scalar multiplication with a unital ring
    pub trait RingModule<K> = AddAbelianGroup + Mul<K, Output=Self> + MulAssign<K> where K: UnitalRing;
    ///An abelian additive group with a distributive scalar multiplication with a field
    pub trait VectorSpace<K> = RingModule<K> + Div<K, Output=Self> + DivAssign<K> where K: Field;
    ///A vector space with a distributive multiplication operation
    pub trait Algebra<K> = VectorSpace<K> + MulMagma + Distributive where K: Field;

    pub trait AffineSpace<K, V> =
        Sized + Clone + Sub<Self, Output=V> + Add<V, Output=Self> + AddAssign<V> + Sub<V, Output=Self> + SubAssign<V>
        where K: Field, V: VectorSpace<K>;

    ///A ring module with a countable basis
    pub trait CountableModule<K> = RingModule<K> + CountableBasis<K> where K: UnitalRing;
    ///A vector space with a countable basis
    pub trait CountableVectorSpace<K> = VectorSpace<K> + CountableBasis<K> where K: Field;
    ///A ring module with a finite dimension
    pub trait FiniteModule<K> = RingModule<K> + FiniteBasis<K> where K: UnitalRing;
    ///A vector space with a finite dimension
    pub trait FiniteVectorSpace<K> = VectorSpace<K> + FiniteBasis<K> where K: Field;

    ///A ring module equipped with a quadradic form
    pub trait QuadradicModule<K> = RingModule<K> + QuadradicForm<K> where K:UnitalRing;
    ///A vector space equipped with a quadradic form
    pub trait QuadradicSpace<K> = VectorSpace<K> + QuadradicForm<K> where K:Field;
    ///A ring module equipped with a bilinear form to the base field
    pub trait BilinearModule<K> = QuadradicModule<K> + BilinearForm<K> where K:UnitalRing;
    ///A vector space equipped with a bilinear form to the base field
    pub trait BilinearSpace<K> = QuadradicSpace<K> + BilinearForm<K> where K:Field;
}
