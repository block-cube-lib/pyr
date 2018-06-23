pub mod color;

pub struct Image<ColorType = color::Color> {
    width: usize,
    height: usize,
    data: Vec<ColorType>,
}

impl<ColorType> Image<ColorType> {

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn data<'a>(&'a self) -> &'a Vec<ColorType> {
        &self.data
    }

    pub fn data_mut<'a>(&'a mut self) -> &'a Vec<ColorType> {
        &mut self.data
    }
}
