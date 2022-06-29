use rand::Rng;
use std::fmt;

#[derive(Debug, Default)]
pub struct Random<T>([T; 3]);

impl<T> Random<T> {
    pub fn new(ts: [T; 3]) -> Self {
        Self(ts)
    }

    fn random(&self) -> &T {
        let i: usize = rand::thread_rng().gen_range(0..=2);
        &self.0[i]
    }

    fn random_mut(&mut self) -> &mut T {
        let i: usize = rand::thread_rng().gen_range(0..=2);
        &mut self.0[i]
    }
}

impl<T> std::ops::Deref for Random<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.random()
    }
}

impl<T> std::ops::DerefMut for Random<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.random_mut()
    }
}

impl<T> From<[T; 3]> for Random<T> {
    fn from(ts: [T; 3]) -> Self {
        Self(ts)
    }
}

impl<T: Copy> TryFrom<&[T]> for Random<T> {
    type Error = std::array::TryFromSliceError;

    fn try_from(value: &[T]) -> Result<Self, Self::Error> {
        let ts: [T; 3] = value.try_into()?;

        Ok(Self(ts))
    }
}

impl<T> fmt::Pointer for Random<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Pointer::fmt(&self.random(), f)
    }
}

impl<T: fmt::Display> fmt::Display for Random<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.random(), f)
    }
}
