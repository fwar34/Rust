// https://zhuanlan.zhihu.com/p/82397973
fn main() {
    let screen = Widget::new(0.0, 0.0, 1920.0, 1080.0);
    let mut window = Widget::new(100.0, 200.0, 50.0, 90.0);

    println!("Screen: {:?}", screen);
    println!("Window: {:?}", window);
    window.dock_left(&screen);
    println!("Docked window: {:?}", window);
}

trait Layoutable {
    fn position(&self) -> (f32, f32);
    fn size(&self) -> (f32, f32);
    fn set_position(&mut self, x: f32, y: f32);
    fn set_size(&mut self, width: f32, height: f32);
}

trait Dockable: Layoutable {
    fn dock_left(&mut self, parent: &dyn Layoutable) {
        let (width, _) = self.size();
        let (_, height) = parent.size();
        self.set_position(0f32, 0f32);
        self.set_size(width, height);
    }
}

#[derive(Copy, Clone, Debug)]
struct Widget {
    pos: (f32, f32),
    size: (f32, f32),
}

impl Widget {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Widget {
        Widget {
            pos: (x, y),
            size: (width, height),
        }
    }
}

impl Layoutable for Widget {
    fn position(&self) -> (f32, f32) {
        self.pos
    }
    fn size(&self) -> (f32, f32) {
        self.size
    }
    fn set_position(&mut self, x: f32, y: f32) {
        self.pos = (x, y);
    }
    fn set_size(&mut self, width: f32, height: f32) {
        self.size = (width, height);
    }
}

impl Dockable for Widget {}

// #[derive(Copy, Clone, Debug)]
