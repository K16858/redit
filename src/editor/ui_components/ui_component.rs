use super::super::Size;
use std::io::Error;

pub trait UIComponent {
    fn mark_redraw(&mut self, value: bool);

    fn needs_redraw(&self) -> bool;

    fn resize(&mut self, size: Size) {
        self.set_size(size);
        self.mark_redraw(true);
    }

    fn set_size(&mut self, size: Size);

    fn render(&mut self, origin_row: usize) {
        if self.needs_redraw() {
            if let Err(err) = self.draw(origin_row) {
                #[cfg(debug_assertions)]
                {
                    panic!("Could not render component: {err:?}");
                }
                #[cfg(not(debug_assertions))]
                {
                    let _ = err;
                }
            } else {
                self.mark_redraw(false);
            }
        }
    }
    fn draw(&mut self, origin_row: usize) -> Result<(), Error>;
}
