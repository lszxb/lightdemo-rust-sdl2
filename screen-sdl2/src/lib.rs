extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Point;
use std::time::Duration;
use std::sync::mpsc;

#[derive(Clone)]
pub struct ColorPoint {
    point: Point,
    color: Color
}

impl ColorPoint {
    pub fn new((x, y): (u32, u32), (r, g, b): (u8, u8, u8)) -> Self {
        ColorPoint {
            point: Point::new(x as i32, y as i32),
            color: Color::RGB(r, g, b)
        }
    }
}

pub enum Task {
    DrawPoint(ColorPoint),
    Clear(Color)
}

pub struct Screen {
    cx: mpsc::Sender<Task>,
    sdl_thread: ::std::thread::JoinHandle<()>
}

impl Screen {
    pub fn new(title: &'static str, width: u32, height: u32) -> Screen {
        let (cx, rx) = mpsc::channel();
        let t = ::std::thread::spawn(move ||{
            let sdl_context = ::sdl2::init().unwrap();
            let video_subsystem = sdl_context.video().unwrap();
        
            let window = video_subsystem.window(title, width, height)
                .position_centered()
                .build()
                .unwrap();
        
            let mut canvas = window.into_canvas().build().unwrap();
        
            canvas.set_draw_color(Color::RGB(255, 255, 255));
            canvas.clear();
            canvas.present();
            let mut event_pump = sdl_context.event_pump().unwrap();
            'running: loop {
                loop {
                    let task: Task = match rx.try_recv() {
                        Ok(x) => x,
                        Err(mpsc::TryRecvError::Empty) => break,
                        Err(_) => return
                    };
                    match task {
                        Task::DrawPoint(p) => {
                            canvas.set_draw_color(p.color);
                            canvas.draw_point(p.point).unwrap();
                        },
                        Task::Clear(c) => {
                            canvas.set_draw_color(c);
                            canvas.clear();
                        }
                    }
                }
                for event in event_pump.poll_iter() {
                    match event {
                        Event::Quit {..} |
                        Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                            break 'running
                        },
                        _ => {}
                    }
                }
                canvas.present();
                ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
            }
        });
        Screen {
            cx: cx,
            sdl_thread: t
        }
    }

    pub fn draw(&self, p: ColorPoint) -> Result<(), mpsc::SendError<Task>> {
        self.cx.send(Task::DrawPoint(p))
    }

    pub fn clear(&self, c: Color) -> Result<(), mpsc::SendError<Task>> {
        self.cx.send(Task::Clear(c))
    }

    pub fn join(self) -> ::std::thread::Result<()> {
        self.sdl_thread.join()
    }
}

