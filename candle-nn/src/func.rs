//! Layers defined by closures.
use candle::{Result, Tensor};
use std::sync::Arc;

/// A layer defined by a simple closure.
#[derive(Clone)]
pub struct Func<'a> {
    #[allow(clippy::type_complexity)]
    f: Arc<dyn 'a + Fn(&Tensor) -> Result<Tensor> + Send + Sync>,
}

impl<'a> std::fmt::Debug for Func<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "func")
    }
}

pub fn func<'a, F>(f: F) -> Func<'a>
where
    F: 'a + Fn(&Tensor) -> Result<Tensor> + Send + Sync,
{
    Func { f: Arc::new(f) }
}

impl<'a> super::Module for Func<'a> {
    fn forward(&self, xs: &Tensor) -> Result<Tensor> {
        (*self.f)(xs)
    }
}

impl<'a> Func<'a> {
    pub fn new<F>(f: F) -> Self
    where
        F: 'a + Fn(&Tensor) -> Result<Tensor> + Send + Sync,
    {
        Self { f: Arc::new(f) }
    }
}
