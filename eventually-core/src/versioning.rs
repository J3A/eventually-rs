//! Contains support for Optimistic Concurrency Control through
//! Versioning.

/// Data type that carries a version for Optimistic Concurrency Control.
pub trait Versioned<T>
where
    T: PartialOrd + Ord + Default + Copy,
{
    /// Minimum version of the data.
    fn min_version(&self) -> T {
        Default::default()
    }

    /// Current version of the data.
    fn version(&self) -> T;
}

impl<T, V> Versioned<V> for Option<T>
where
    T: Versioned<V>,
    V: PartialOrd + Ord + Default + Copy,
{
    #[inline]
    fn version(&self) -> V {
        self.as_ref().map(Versioned::version).unwrap_or_default()
    }
}
