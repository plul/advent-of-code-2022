use std::cmp::max;
use std::cmp::min;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Sub;

#[derive(PartialEq, Eq, Debug, Clone, Copy, Default, Hash)]
pub struct Vector2D<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vector2D<T> {
    pub fn chebyshev_distance(self, other: Self) -> T
    where
        T: Clone + Ord + Sub<Output = T>,
    {
        let diff_x = max(self.x.clone(), other.x.clone()) - min(self.x, other.x);
        let diff_y = max(self.y.clone(), other.y.clone()) - min(self.y, other.y);
        max(diff_x, diff_y)
    }

    pub fn clamp_x(mut self, min: T, max: T) -> Self
    where
        T: Ord,
    {
        self.x = self.x.clamp(min, max);
        self
    }

    pub fn clamp_y(mut self, min: T, max: T) -> Self
    where
        T: Ord,
    {
        self.y = self.y.clamp(min, max);
        self
    }
}

impl<T> From<(T, T)> for Vector2D<T> {
    fn from(value: (T, T)) -> Self {
        Vector2D {
            x: value.0,
            y: value.1,
        }
    }
}

impl<T, U> AddAssign<U> for Vector2D<T>
where
    U: Into<Self>,
    T: AddAssign,
{
    fn add_assign(&mut self, rhs: U) {
        let rhs = rhs.into();
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T, U> Add<U> for Vector2D<T>
where
    U: Into<Self>,
    T: Add<Output = T>,
{
    type Output = Self;
    fn add(mut self, rhs: U) -> Self {
        let rhs = rhs.into();
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
        self
    }
}

impl<T, U> Sub<U> for Vector2D<T>
where
    U: Into<Self>,
    T: Sub<Output = T>,
{
    type Output = Self;
    fn sub(mut self, rhs: U) -> Self {
        let rhs = rhs.into();
        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
        self
    }
}
