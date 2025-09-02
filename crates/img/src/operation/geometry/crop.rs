use crate::{
    image::Image,
    pipe::{
        FromPipe,
        FromPipePar,
        Pipe,
    },
    pixel::Pixel,
    primitive::{
        margin::Margin,
        offset::Offset,
        size::{
            self,
        },
    },
};

pub fn crop_pipe<S>(
    source: S,
    margin: Margin,
) -> Result<impl Pipe<Item = Pixel>, size::CreationError>
where
    S: Pipe,
    S::Item: AsRef<Pixel>,
{
    let size = source.size();
    let new_size = size.apply_margin(margin)?;

    Ok(source.remap(
        move |pipe, point| {
            let top_left = margin.top_left();
            let original_point =
                point.offset_by(Offset::new(top_left.x() as isize, top_left.y() as isize)).unwrap();

            *pipe.get(original_point).expect("bug in pipe implementation").as_ref()
        },
        new_size,
    ))
}

pub fn crop(image: &Image, margin: Margin) -> Result<Image, size::CreationError> {
    let pipe = crop_pipe(image.pipe(), margin)?;
    let image = Image::from_pipe(pipe);

    Ok(image)
}

#[cfg(feature = "parallel")]
pub fn crop_par(image: &Image, margin: Margin) -> Result<Image, size::CreationError> {
    let pipe = crop_pipe(image.pipe(), margin)?;
    let image = Image::from_pipe_par(pipe);

    Ok(image)
}
