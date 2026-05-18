use std::num::NonZeroU32;
use std::sync::Arc;

use softbuffer::{Context, Surface};
use tiny_skia::{Color, Paint, Pixmap, Rect, Transform};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowId};

use crate::painting::{DisplayCommand, DisplayList};

/// 埋め込みフォントデータ（Ubuntu Regular）。
const FONT_BYTES: &[u8] = include_bytes!("../assets/fonts/Ubuntu-R.ttf");

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
            WindowEvent::Resized(_) => {
                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }
            WindowEvent::RedrawRequested => {
                let window = self.window.as_ref().unwrap();
                let surface = self.surface.as_mut().unwrap();
                let size = window.inner_size();
                let w = NonZeroU32::new(size.width).unwrap();
                let h = NonZeroU32::new(size.height).unwrap();
                surface.resize(w, h).unwrap();

                // パス1: SolidColor を tiny-skia の Pixmap に描画する。
                let mut pixmap = Pixmap::new(size.width, size.height).unwrap();
                pixmap.fill(Color::WHITE);
                for cmd in &self.display_list {
                    if let DisplayCommand::SolidColor(color, rect) = cmd {
                        let mut paint = Paint::default();
                        paint.set_color_rgba8(color.r, color.g, color.b, 255);
                        if let Some(skia_rect) =
                            Rect::from_xywh(rect.x, rect.y, rect.width, rect.height)
                        {
                            pixmap.fill_rect(skia_rect, &paint, Transform::identity(), None);
                        }
                    }
                }

                // Pixmap の内容を softbuffer のピクセルバッファに転送する。
                let mut buf = surface.buffer_mut().unwrap();
                for (i, pixel) in pixmap.pixels().iter().enumerate() {
                    buf[i] = ((pixel.red() as u32) << 16)
                        | ((pixel.green() as u32) << 8)
                        | (pixel.blue() as u32);
                }

                // パス2: Text を fontdue でラスタライズしてバッファに直接書き込む。
                // アルファブレンディングで背景色と合成することでアンチエイリアスを実現する。
                let font = fontdue::Font::from_bytes(FONT_BYTES, fontdue::FontSettings::default())
                    .unwrap();
                for cmd in &self.display_list {
                    if let DisplayCommand::Text {
                        text,
                        x,
                        y,
                        color,
                        font_size,
                    } = cmd
                    {
                        let mut cursor_x = *x;
                        for ch in text.chars() {
                            let (metrics, bitmap) = font.rasterize(ch, *font_size);
                            for row in 0..metrics.height {
                                for col in 0..metrics.width {
                                    let alpha = bitmap[row * metrics.width + col];
                                    let px = cursor_x as usize + col;
                                    // ymin と height でグリフのベースラインを揃える。
                                    let py = (*y as i32 + row as i32
                                        - (metrics.ymin + metrics.height as i32))
                                        as usize;
                                    if px < size.width as usize && py < size.height as usize {
                                        let idx = py * size.width as usize + px;
                                        let a = alpha as u32;
                                        let bg = buf[idx];
                                        let bg_r = (bg >> 16) & 0xff;
                                        let bg_g = (bg >> 8) & 0xff;
                                        let bg_b = bg & 0xff;
                                        let r = (color.r as u32 * a + bg_r * (255 - a)) / 255;
                                        let g = (color.g as u32 * a + bg_g * (255 - a)) / 255;
                                        let b = (color.b as u32 * a + bg_b * (255 - a)) / 255;
                                        buf[idx] = (r << 16) | (g << 8) | b;
                                    }
                                }
                            }
                            cursor_x += metrics.advance_width;
                        }
                    }
                }

                buf.present().unwrap();
            }
            _ => {}
        }
    }
}
