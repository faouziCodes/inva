use std::io::Stdout;

pub trait Component {
    fn draw(&self, draw_to: &mut Stdout);
}
