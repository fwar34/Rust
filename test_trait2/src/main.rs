// https://zhuanlan.zhihu.com/p/76740667
fn main() {
    println!("Hello, world!");
}

trait Stream {
    fn read(&self, buffer: &mut [u8]) -> usize;
    fn write(&mut self, buffer: &[u8]) ->usize;
}

struct Console;
struct FileStream;

impl Console { ... }
impl FileStream { ... }

impl Stream for Console {
    fn read(&self, buffer: &mut [u8]) -> usize { ... }
    ...
}

impl Stream for FileStream { ... }

let BUFFER_SIZE: u32 = 56635;

fn some_algorithm_dynamic(stream: &mut dyn Stream) {
    let mut buffer = [0u8; BUFFER_SIZE];
    stream.read(&mut buffer);
}

fn some_algorithm_static<T: Stream>(stream: &mut T) {
    let mut buffer = [0u8; BUFFER_SIZE];
    stream.read(&mut buffer);
}
