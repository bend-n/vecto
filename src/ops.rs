use crate::Vector2;
use core::ops::{
    Add as add, AddAssign as add_assign, Div as div, DivAssign as div_assign, Mul as mul,
    MulAssign as mul_assign, Neg, Rem as rem, RemAssign as rem_assign, Sub as sub,
    SubAssign as sub_assign,
};

macro_rules! op {
    ($name:ident) => {
        impl<T: $name<T, Output = T>> $name<Vector2<T>> for Vector2<T> {
            type Output = Vector2<T>;

            fn $name(self, rhs: Vector2<T>) -> Self::Output {
                Self::new(self.x.$name(rhs.x), self.y.$name(rhs.y))
            }
        }

        impl<T: Copy + $name<T, Output = T>> $name<&Vector2<T>> for Vector2<T> {
            type Output = Vector2<T>;

            fn $name(self, rhs: &Vector2<T>) -> Self::Output {
                Self::new(self.x.$name(rhs.x), self.y.$name(rhs.y))
            }
        }

        impl<T: Copy + $name<T, Output = T>> $name<T> for Vector2<T> {
            type Output = Vector2<T>;
            fn $name(self, rhs: T) -> Self::Output {
                Self::new(self.x.$name(rhs), self.y.$name(rhs))
            }
        }

        impl<T: Copy + $name<T, Output = T>> $name<&T> for Vector2<T> {
            type Output = Vector2<T>;
            fn $name(self, rhs: &T) -> Self::Output {
                Self::new(self.x.$name(*rhs), self.y.$name(*rhs))
            }
        }
    };
}
op!(add);
op!(div);
op!(mul);
op!(rem);
op!(sub);

macro_rules! assign {
    ($name:ident, $op:ident) => {
        impl<T: $name<T>> $name<Vector2<T>> for Vector2<T> {
            fn $name(&mut self, rhs: Vector2<T>) {
                self.x.$name(rhs.x);
                self.y.$name(rhs.y);
            }
        }

        impl<T: Copy + $name<T>> $name<&Vector2<T>> for Vector2<T> {
            fn $name(&mut self, rhs: &Vector2<T>) {
                self.x.$name(rhs.x);
                self.y.$name(rhs.y);
            }
        }

        impl<T: Copy + $name<T>> $name<T> for Vector2<T> {
            fn $name(&mut self, rhs: T) {
                self.x.$name(rhs);
                self.y.$name(rhs);
            }
        }

        impl<T: Copy + $name<T>> $name<&T> for Vector2<T> {
            fn $name(&mut self, rhs: &T) {
                self.x.$name(*rhs);
                self.y.$name(*rhs);
            }
        }
    };
}
assign!(add_assign, add);
assign!(div_assign, div);
assign!(mul_assign, mul);
assign!(rem_assign, rem);
assign!(sub_assign, sub);

impl<T: Neg<Output = T>> Neg for Vector2<T> {
    type Output = Vector2<T>;

    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y)
    }
}
