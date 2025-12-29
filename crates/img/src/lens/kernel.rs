use std::{
    collections::HashMap,
    marker::PhantomData,
    sync::{
        Arc,
        RwLock,
    },
    thread::ThreadId,
};

use thiserror::Error;

use crate::{
    component::{
        kernel::Kernel,
        primitive::{
            Point,
            Size,
            SizeCreationError,
        },
    },
    error::{
        IndexError,
        IndexResult,
    },
    lens::Lens,
    prelude::{
        Area,
        Offset,
    },
};

#[derive(Debug, Error)]
pub enum CreationError {
    #[error("kernel's width is too big")]
    KernelTooBigX,
    #[error("kernel's height is too big")]
    KernelTooBigY,
}

#[derive(Clone)]
pub struct KernelLens<S, K, T> {
    source: S,
    kernel: K,
    working_area: Area,
    _phantom_data: PhantomData<T>,
}

pub struct KernelLensPar<S, F, K, T> {
    source: S,
    kernel_factory: Arc<F>,
    kernels: RwLock<HashMap<ThreadId, K>>,
    working_area: Area,
    _phantom_data: PhantomData<T>,
    _kernel: PhantomData<K>,
}

unsafe impl<S, F, K, T> Sync for KernelLensPar<S, F, K, T>
where
    S: Sync,
    F: Sync,
    K: Send,
{
}

impl<S, K, T> KernelLens<S, K, T>
where
    S: Lens,
    K: Kernel<S::Item, T>,
{
    pub fn new(source: S, kernel: K) -> Result<Self, CreationError> {
        let margin = kernel.margin();

        let size = source.size().shrink_by_margin(kernel.margin()).map_err(|e| match e {
            SizeCreationError::WidthZero => CreationError::KernelTooBigX,
            SizeCreationError::HeightZero => CreationError::KernelTooBigY,
            _ => unreachable!("unexpected error returned from shrink_by_margin"),
        })?;

        let top_left =
            Point::new(margin.left(), margin.top()).expect("unexpected error in Point::new");

        let working_area = Area::new(size, top_left);

        Ok(Self { source, kernel, working_area, _phantom_data: Default::default() })
    }
}

impl<S, F, K, T> KernelLensPar<S, F, K, T>
where
    S: Lens,
    F: Fn() -> K + Send + Sync,
    K: Kernel<S::Item, T>,
{
    pub fn new(source: S, kernel_factory: F) -> Result<Self, CreationError> {
        let kernel = kernel_factory();
        let margin = kernel.margin();

        let size = source.size().shrink_by_margin(margin).map_err(|e| match e {
            SizeCreationError::WidthZero => CreationError::KernelTooBigX,
            SizeCreationError::HeightZero => CreationError::KernelTooBigY,
            _ => unreachable!("unexpected error returned from shrink_by_margin"),
        })?;

        let top_left =
            Point::new(margin.left(), margin.top()).expect("unexpected error in Point::new");

        let working_area = Area::new(size, top_left);

        let thread_id = std::thread::current().id();
        let mut kernels = HashMap::new();
        kernels.insert(thread_id, kernel);
        Ok(Self {
            source,
            kernel_factory: Arc::new(kernel_factory),
            kernels: RwLock::new(kernels),
            working_area,
            _phantom_data: Default::default(),
            _kernel: Default::default(),
        })
    }
}

impl<S, K, T> Lens for KernelLens<S, K, T>
where
    S: Lens,
    K: Kernel<S::Item, T>,
{
    type Item = T;

    fn look(&self, point: Point) -> IndexResult<Self::Item> {
        if !self.size().contains(&point) {
            return Err(IndexError::OutOfBounds);
        }
        let offset: Offset = self.working_area.top_left().into();
        let source_point = point.translate(offset).expect("unexpected error in Point::translate");

        self.kernel.apply(&self.source, source_point)
    }

    fn size(&self) -> Size {
        self.working_area.size()
    }
}

impl<S, F, K, T> Lens for KernelLensPar<S, F, K, T>
where
    S: Lens,
    F: Fn() -> K + Send + Sync,
    K: Kernel<S::Item, T>,
{
    type Item = T;

    fn look(&self, point: Point) -> IndexResult<Self::Item> {
        if !self.size().contains(&point) {
            return Err(IndexError::OutOfBounds);
        }
        let offset: Offset = self.working_area.top_left().into();
        let source_point = point.translate(offset).expect("unexpected error in Point::translate");

        let thread_id = std::thread::current().id();

        if let Some(kernel) = self.kernels.read().unwrap().get(&thread_id) {
            return kernel.apply(&self.source, source_point);
        }

        let mut kernels = self.kernels.write().unwrap();

        let kernel = kernels.entry(thread_id).or_insert_with(|| (self.kernel_factory)());

        kernel.apply(&self.source, source_point)
    }

    fn size(&self) -> Size {
        self.working_area.size()
    }
}
