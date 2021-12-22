use std::fmt;
use std::ops::{Add, AddAssign, Mul, Sub, SubAssign};

#[derive(Eq, PartialEq, Hash)]
pub struct Point<T, const D: usize> {
    data: [T; D],
}

impl<T> Point<T, 2> {
    pub fn new(x: T, y: T) -> Point<T, 2> {
        Point { data: [x, y] }
    }

    pub fn x(&self) -> &T {
        &self.data[0]
    }

    pub fn y(&self) -> &T {
        &self.data[1]
    }

    pub fn tuple_ref(&self) -> (&T, &T) {
        (&self.data[0], &self.data[1])
    }

    pub fn tuple_copy(&self) -> (T, T)
    where
        T: Copy,
    {
        (self.data[0], self.data[1])
    }
}
pub type Point2D<T> = Point<T, 2>;

impl<T> Point<T, 3> {
    pub fn new(x: T, y: T, z: T) -> Point<T, 3> {
        Point { data: [x, y, z] }
    }

    pub fn x(&self) -> &T {
        &self.data[0]
    }

    pub fn y(&self) -> &T {
        &self.data[1]
    }

    pub fn z(&self) -> &T {
        &self.data[2]
    }

    pub fn tuple_ref(&self) -> (&T, &T, &T) {
        (&self.data[0], &self.data[1], &self.data[2])
    }

    pub fn tuple_copy(&self) -> (T, T, T)
    where
        T: Copy,
    {
        (self.data[0], self.data[1], self.data[2])
    }
}
pub type Point3D<T> = Point<T, 3>;

impl<T, const D: usize> Point<T, D>
where
    T: Default + AddAssign + Ord + Sub<Output = T> + Mul<Output = T> + Copy,
{
    pub fn distance_squared(&self, other: &Point<T, D>) -> T {
        let mut sum = T::default();
        for i in 0..D {
            let delta = self.data[i].max(other.data[i]) - self.data[i].min(other.data[i]);
            sum += delta * delta;
        }
        sum
    }

    pub fn magnitude_squared(&self) -> T {
        Point::<T, D>::origin().distance_squared(self)
    }

    pub fn manhattan_distance(&self, other: &Point<T, D>) -> T {
        let mut sum = T::default();
        for i in 0..D {
            let delta = self.data[i].max(other.data[i]) - self.data[i].min(other.data[i]);
            sum += delta;
        }
        sum
    }
}

impl<T, const D: usize> Point<T, D>
where
    T: Default + Copy,
{
    pub fn origin() -> Self {
        Point {
            data: [T::default(); D],
        }
    }
}

impl<T, const D: usize> Clone for Point<T, D>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Point {
            data: self.data.clone(),
        }
    }
}

impl<T, const D: usize> Copy for Point<T, D> where T: Copy {}

impl<T, const D: usize> fmt::Debug for Point<T, D>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut dbg_real = f.debug_tuple("");
        let mut dbg_ref = &mut dbg_real;
        for i in &self.data {
            dbg_ref = dbg_ref.field(i)
        }
        dbg_ref.finish()
    }
}

impl<T, const D: usize> Add for Point<T, D>
where
    T: Add<Output = T> + Copy + Default,
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut result = [T::default(); D];

        for i in 0..D {
            result[i] = self.data[i] + other.data[i];
        }

        Self { data: result }
    }
}

impl<T, const D: usize> AddAssign for Point<T, D>
where
    T: AddAssign + Copy,
{
    fn add_assign(&mut self, other: Self) {
        for i in 0..D {
            self.data[i] += other.data[i];
        }
    }
}

impl<T, const D: usize> Sub for Point<T, D>
where
    T: Sub<Output = T> + Copy + Default,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let mut result = [T::default(); D];

        for i in 0..D {
            result[i] = self.data[i] - other.data[i];
        }

        Self { data: result }
    }
}

impl<T, const D: usize> SubAssign for Point<T, D>
where
    T: SubAssign + Copy,
{
    fn sub_assign(&mut self, other: Self) {
        for i in 0..D {
            self.data[i] -= other.data[i];
        }
    }
}

impl<T> Add<(T, T)> for Point<T, 2>
where
    T: Add<Output = T> + Copy + Default,
{
    type Output = Self;

    fn add(self, other: (T, T)) -> Self {
        let mut result = [T::default(); 2];

        result[0] = self.data[0] + other.0;
        result[1] = self.data[1] + other.1;

        Self { data: result }
    }
}

impl<T> AddAssign<(T, T)> for Point<T, 2>
where
    T: AddAssign + Copy,
{
    fn add_assign(&mut self, other: (T, T)) {
        self.data[0] += other.0;
        self.data[1] += other.1;
    }
}

impl<T> Sub<(T, T)> for Point<T, 2>
where
    T: Sub<Output = T> + Copy + Default,
{
    type Output = Self;

    fn sub(self, other: (T, T)) -> Self {
        let mut result = [T::default(); 2];

        result[0] = self.data[0] - other.0;
        result[1] = self.data[1] - other.1;

        Self { data: result }
    }
}

impl<T> SubAssign<(T, T)> for Point<T, 2>
where
    T: SubAssign + Copy,
{
    fn sub_assign(&mut self, other: (T, T)) {
        self.data[0] -= other.0;
        self.data[1] -= other.1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ctor() {
        let p = Point::new(1, 1000);
        assert_eq!(&1, p.x());
        assert_eq!(&1000, p.y());
    }

    #[test]
    fn test_add_points() {
        let p1 = Point::new(1, 100);
        let p2 = Point::new(-10, 50);
        let added = p1 + p2;
        assert_eq!(&-9, added.x());
        assert_eq!(&150, added.y());
    }

    #[test]
    fn test_add_tuple() {
        let p1 = Point::new(1, 100);
        let p2 = (9, -200);
        let added = p1 + p2;
        assert_eq!(&10, added.x());
        assert_eq!(&-100, added.y());
    }
}
