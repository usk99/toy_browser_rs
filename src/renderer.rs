use std::num::NonZeroU32;
use std::sync::Arc;

use softbuffer::{Context, Surface};
use tiny_skia::{Color, Paint, Pixmap, Rect, Transform};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowId};

use crate::painting::{DisplayCommand, DisplayList};

/// ウィンドウの初期幅（論理ピクセル）。
const WIDTH: u32 = 800;
/// ウィンドウの初期高さ（論理ピクセル）。
const HEIGHT: u32 = 600;

/// イベントループを起動してウィンドウを表示する。
pub fn run(display_list: DisplayList) {
    let event_loop = EventLoop::new().unwrap();
    let mut app = App::new(display_list);
    event_loop.run_app(&mut app).ok();
}

/// winit イベントループで管理するアプリ状態。
struct App {
    /// 描画命令リスト。
    display_list: DisplayList,
    /// ウィンドウ。`resumed` 後に初期化される。
    window: Option<Arc<Window>>,
    /// ピクセルバッファのサーフェス。`resumed` 後に初期化される。
    surface: Option<Surface<Arc<Window>, Arc<Window>>>,
}

impl App {
    /// 描画命令リストを受け取って `App` を生成する。
    fn new(display_list: DisplayList) -> Self {
        Self {
            display_list,
            window: None,
            surface: None,
        }
    }
}

impl ApplicationHandler for App {
    /// ウィンドウが作れる状態になったら呼ばれる。ウィンドウとサーフェスを初期化する。
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Arc::new(
            event_loop
                .create_window(
                    Window::default_attributes()
                        .with_title("toy browser")
                        .with_inner_size(winit::dpi::LogicalSize::new(WIDTH, HEIGHT)),
                )
                .unwrap(),
        );
        let context = Context::new(Arc::clone(&window)).unwrap();
        let surface = Surface::new(&context, Arc::clone(&window)).unwrap();
        self.window = Some(window);
        self.surface = Some(surface);
    }

    /// ウィンドウイベントを処理する。描画・終了を担当する。
    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                let window = self.window.as_ref().unwrap();
                let surface = self.surface.as_mut().unwrap();
                let size = window.inner_size();
                let w = NonZeroU32::new(size.width).unwrap();
                let h = NonZeroU32::new(size.height).unwrap();
                surface.resize(w, h).unwrap();

                // tiny-skia でピクセルマップに描画
                let mut pixmap = Pixmap::new(size.width, size.height).unwrap();
                pixmap.fill(Color::WHITE); // 背景を白で塗りつぶす

                for cmd in &self.display_list {
                    match cmd {
                        DisplayCommand::SolidColor(color, rect) => {
                            let mut paint = Paint::default();
                            paint.set_color_rgba8(color.r, color.g, color.b, 255);
                            if let Some(skia_rect) =
                                Rect::from_xywh(rect.x, rect.y, rect.width, rect.height)
                            {
                                pixmap.fill_rect(skia_rect, &paint, Transform::identity(), None);
                            }
                        }
                    }
                }

                // softbuffer にピクセルを転送
                let mut buf = surface.buffer_mut().unwrap();
                for (i, pixel) in pixmap.pixels().iter().enumerate() {
                    buf[i] = ((pixel.red() as u32) << 16)
                        | ((pixel.green() as u32) << 8)
                        | (pixel.blue() as u32);
                }
                buf.present().unwrap();
            }
            _ => {}
        }
    }
}
