//! Contains support for Optimistic Concurrency Control through
//! Versioning.

/// Data type that carries a version for Optimistic Concurrency Control.
pub trait Versioned<T>
where
    T: PartialOrd + Ord + Default,
{
    /// Minimum version of the data.
    fn min_version(&self) -> T;

    /// Current version of the data.
    fn version(&self) -> T;
}

impl<T, V> Versioned<V> for Option<T>
where
    T: Versioned<V>,
    V: PartialOrd + Ord + Default,
{
    #[inline]
    fn version(&self) -> V {
        self.as_ref().map(Versioned::version).unwrap_or_default()
    }

    fn min_version(&self) -> V {
        todo!()
    }
}
