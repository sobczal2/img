use std::{cell::RefCell};

use crate::{error::IndexResult, pipe::{swap::SwapPipe, Pipe}, pixel::Pixel, primitive::{point::Point, size::Size}};

pub struct ColorsPipe<P> {
    source: P,
    red_swap_pipe: SwapPipe<OneshotPipe<u8>>,
    red_pipe: Box<dyn Pipe<Item = u8>>,
    green_swap_pipe: SwapPipe<OneshotPipe<u8>>,
    green_pipe: Box<dyn Pipe<Item = u8>>,
    blue_swap_pipe: SwapPipe<OneshotPipe<u8>>,
    blue_pipe: Box<dyn Pipe<Item = u8>>,
    alpha_swap_pipe: SwapPipe<OneshotPipe<u8>>,
    alpha_pipe: Box<dyn Pipe<Item = u8>>,
}

pub type PipeFactory = Box<dyn Fn(SwapPipe<OneshotPipe<u8>>) -> Box<dyn Pipe<Item = u8>>>;

impl<P> ColorsPipe<P>
    where P: Pipe
{
    pub fn new(
        source: P,
        red_pipe_factory: PipeFactory,
        green_pipe_factory: PipeFactory,
        blue_pipe_factory: PipeFactory,
        alpha_pipe_factory: PipeFactory
    ) -> Self
    {
        let red_swap_pipe = SwapPipe::new(OneshotPipe::new(None, source.size()));
        let red_pipe = (red_pipe_factory)(red_swap_pipe.clone());

        let green_swap_pipe = SwapPipe::new(OneshotPipe::new(None, source.size()));
        let green_pipe = (green_pipe_factory)(green_swap_pipe.clone());

        let blue_swap_pipe = SwapPipe::new(OneshotPipe::new(None, source.size()));
        let blue_pipe = (blue_pipe_factory)(blue_swap_pipe.clone());

        let alpha_swap_pipe = SwapPipe::new(OneshotPipe::new(None, source.size()));
        let alpha_pipe = (alpha_pipe_factory)(alpha_swap_pipe.clone());

        Self {
            source,
            red_swap_pipe,
            red_pipe,
            green_swap_pipe,
            green_pipe,
            blue_swap_pipe,
            blue_pipe,
            alpha_swap_pipe,
            alpha_pipe
        }
    }
}

pub struct OneshotPipe<T> {
    value: RefCell<Option<T>>,
    size: Size,
}

impl<T> OneshotPipe<T> {
    fn new(value: Option<T>, size: Size) -> Self {
        Self { value: value.into(), size }
    }
}

impl<T> Pipe for OneshotPipe<T> {
    type Item = T;

    fn get(&self, _point: Point) -> IndexResult<Self::Item> {
        Ok(self.value.take().unwrap())
    }

    fn size(&self) -> Size {
        self.size
    }
}

impl<P> Pipe for ColorsPipe<P>
    where P: Pipe, P::Item: AsRef<Pixel>, 

{
    type Item = Pixel;

    fn get(&self, point: Point) -> IndexResult<Self::Item> {
        let item = self.source.get(point)?;

        let red_pipe = OneshotPipe { value: RefCell::new(Some(item.as_ref().r())), size: self.source.size() };
        let green_pipe = OneshotPipe { value: RefCell::new(Some(item.as_ref().g())), size: self.source.size() };
        let blue_pipe = OneshotPipe { value: RefCell::new(Some(item.as_ref().b())), size: self.source.size() };
        let alpha_pipe = OneshotPipe { value: RefCell::new(Some(item.as_ref().a())), size: self.source.size() };

        self.red_swap_pipe.swap(red_pipe);
        let red = self.red_pipe.get(point)?;
        self.green_swap_pipe.swap(green_pipe);
        let green = self.green_pipe.get(point)?;
        self.blue_swap_pipe.swap(blue_pipe);
        let blue = self.blue_pipe.get(point)?;
        self.alpha_swap_pipe.swap(alpha_pipe);
        let alpha = self.alpha_pipe.get(point)?;

        Ok(Pixel::new([red, green, blue, alpha]))
    }

    fn size(&self) -> Size {
        self.source.size()
    }
}
