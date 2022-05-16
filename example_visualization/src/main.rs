#![feature(array_windows)]

use std::time::Instant;
use std::sync::atomic::{AtomicU64, Ordering};
use cannoli::{Cannoli, create_cannoli};
use snuffles::{CameraMode, Vsync, Msaa, EventHandler, Window, Vertex};
use snuffles::{Persist, DrawCommand};

const WIDTH:  usize = 800;
const HEIGHT: usize = 600;

/// Our handlers for QEMU devents
struct Tracer {
}

static mut RAW_DATA: [u8; WIDTH * HEIGHT * 4] = [0u8; WIDTH * HEIGHT * 4];

static LOADS:  AtomicU64 = AtomicU64::new(0);
static STORES: AtomicU64 = AtomicU64::new(0);
static INSTRS: AtomicU64 = AtomicU64::new(0);

#[derive(Clone, Copy)]
enum Trace {
    Load(u32),
    Store(u32),
    Instr(u32),
}

impl Tracer {
    #[inline]
    fn set_pixel(&mut self, addr: u32, color: (u8, u8, u8)) {
        const ADDR_MAX: u32 = 0x1000_0000;

        // Get the number of target bytes per host pixel
        let scale = unsafe { ADDR_MAX as f64 / (RAW_DATA.len()) as f64 };

        // Get the pixel index
        let pixel = addr as f64 / scale;

        // Round the pixel index down
        let pixel = (pixel as usize) & !3;

        unsafe {
            RAW_DATA[pixel + 0] |= color.2; // B
            RAW_DATA[pixel + 1] |= color.1; // G
            RAW_DATA[pixel + 2] |= color.0; // R
            RAW_DATA[pixel + 3] |= 0x00;
        }
    }
}

impl Cannoli for Tracer {
    type Trace = Trace;
    type Context = ();

    fn init(_: u64) -> (Self, Self::Context) {
        (Tracer {
        }, ())
    }

    fn exec(_ctxt: &Self::Context, pc: u64) -> Option<Self::Trace> {
        Some(Trace::Instr(pc as u32))
    }

    fn read(_ctxt: &Self::Context, _pc: u64, addr: u64, _val: u64)
            -> Option<Self::Trace> {
        Some(Trace::Load(addr as u32))
    }

    fn write(_ctxt: &Self::Context, _pc: u64, addr: u64, _val: u64)
            -> Option<Self::Trace> {
        Some(Trace::Store(addr as u32))
    }

    fn trace(&mut self, _ctxt: &Self::Context, trace: &[Self::Trace]) {
        let mut loads  = 0;
        let mut stores = 0;
        let mut instrs = 0;

        for event in trace {
            match *event {
                Trace::Load(addr) => {
                    self.set_pixel(addr as u32, (0x00, 0xff, 0x00));
                    loads += 1;
                }
                Trace::Store(addr) => {
                    self.set_pixel(addr as u32, (0xff, 0x00, 0x00));
                    stores += 1;
                }
                Trace::Instr(_addr) => {
                    instrs += 1;
                }
            }
        }

        LOADS.fetch_add(loads, Ordering::Relaxed);
        STORES.fetch_add(stores, Ordering::Relaxed);
        INSTRS.fetch_add(instrs, Ordering::Relaxed);
    }
}

#[derive(PartialOrd, Ord, PartialEq, Eq)]
enum GraphType {
    Instr,
    Load,
    Store,
}

/// A very bad graphing API
struct ShitGraph {
    x: u32,
    y: u32,

    width:  u32,
    height: u32,

    graph_type: GraphType,

    data: Vec<(Instant, u64)>,
}

impl ShitGraph {
    fn new(x: u32, y: u32, width: u32, height: u32, graph_type: GraphType)
            -> Self {
        Self {
            x, y, width, height, graph_type,
            data: Vec::new(),
        }
    }

    fn draw(&mut self, window: &mut Window<Renderer>) {
        const AVG_WINDOW: usize = 30;

        if self.graph_type == GraphType::Instr {
            // Log the number of instructions executed
            let instrs = INSTRS.load(Ordering::Relaxed);
            self.data.push((Instant::now(), instrs));
        } else if self.graph_type == GraphType::Load {
            // Log the number of loads
            let instrs = LOADS.load(Ordering::Relaxed);
            self.data.push((Instant::now(), instrs));
        } else if self.graph_type == GraphType::Store {
            // Log the number of stores
            let instrs = STORES.load(Ordering::Relaxed);
            self.data.push((Instant::now(), instrs));
        }

        // Limit data log to `width` entries, this is effectively when we have
        // one pixel per `x` data point, beyond this we don't really care
        while self.data.len() > self.width as usize {
            self.data.remove(0);
        }

        // Window size
        if self.data.len() <= AVG_WINDOW {
            return;
        }

        // Get the time width of the X axis
        let start  = self.data.iter().next().unwrap().0;
        let end    = self.data.iter().last().unwrap().0;
        let xrange = (end - start).as_secs_f32();

        let mut draw: Vec<(f32, f32)> = Vec::new();
        let mut last = 0.;
        for (ii, &(time, data)) in self.data.iter()
                .enumerate().skip(AVG_WINDOW) {
            // The window to normalize against
            let old_record = self.data[ii - AVG_WINDOW];
            let window = (time - old_record.0).as_secs_f32();
            let delta  = data - old_record.1;
            let ips    = delta as f32 / window;
            last = ips;

            let time_delta = (time - start).as_secs_f32() / xrange;
            draw.push((time_delta * self.width as f32, ips));
        }

        // Normalize the data
        let max = draw.iter().max_by(|(_, y1), (_, y2)|
            y1.partial_cmp(y2).unwrap())
            .unwrap().1;
        draw.iter_mut().for_each(|(_, y)| {
            *y = *y / max as f32 * self.height as f32
        });

        let mut verts = Vec::new();
        for &[(x1, y1), (x2, y2)] in draw.array_windows::<2>() {
            verts.push(Vertex::new(self.x as f32 + x1,
                    self.y as f32 + y1, 0., 0, 0, 255));
            verts.push(Vertex::new(self.x as f32 + x2,
                    self.y as f32 + y2, 0., 0, 0, 255));
        }

        let last = last / 1e6;
        let txt = match self.graph_type {
            GraphType::Load  => format!("{:10.4} Mload/sec", last),
            GraphType::Store => format!("{:10.4} Mstore/sec", last),
            GraphType::Instr => format!("{:10.4} Minst/sec", last),
        };

        window.push_text(self.x as f32 + 2., self.y as f32 + 10., 1., 1., 1., txt);

        let buffer = window.create_vertex_buffer(verts.as_slice());
        window.push_command(
            DrawCommand::Lines(Persist::No,
                buffer, 0..verts.len() as u32));
    }
}

/// Our structure for rendering and stuff
struct Renderer {
    graphs: Vec<ShitGraph>,
}

impl EventHandler for Renderer {
    fn create(_window: &mut Window<Self>) -> Self {
        Self {
            graphs: vec![
                ShitGraph::new(  0, 0, 200, 200, GraphType::Instr),
                ShitGraph::new(200, 0, 200, 200, GraphType::Load),
                ShitGraph::new(400, 0, 200, 200, GraphType::Store),
            ],
        }
    }

    fn should_redraw(&mut self, window: &mut Window<Self>) {
        // Something changed!
        window.request_redraw(true);
    }

    // Called whenver we requested a redraw
    fn render_ui(&mut self, window: &mut Window<Self>) {
        unsafe {
            window.write_background_texture(&RAW_DATA[..]);

            // Zero out memory access info
            RAW_DATA.iter_mut().for_each(|x| *x = x.saturating_sub(8));
        }

        for graph in &mut self.graphs {
            graph.draw(window);
        }
    }
}

fn main() {
    std::thread::spawn(|| {
        // Create Cannoli handler!
        create_cannoli::<Tracer>(2).expect("Cannoli handler returned error");
    });

    Window::<Renderer>::new("Hello world",
        WIDTH as u32, HEIGHT as u32, Msaa::None, Vsync::On)
        .expect("Failed to create window")
        .camera_mode(CameraMode::Pannable2d)
        .run();
}

