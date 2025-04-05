use crate::core::graphics::{Color, Point, Rect};
use crate::core::event::{Event, MouseEvent, MouseButton};

#[derive(Debug)]
pub struct Button {
    pub rect: Rect,
    pub label: String,
    pub background_color: Color,
    pub text_color: Color,
    pub on_click: Option<Box<dyn FnMut()>>,
    state: ButtonState,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum ButtonState {
    Normal,
    Hovered,
    Pressed,
}

impl Button {
    pub fn new(rect: Rect, label: String) -> Self {
        Button {
            rect,
            label,
            background_color: Color::rgb(0.8, 0.8, 0.8), // Açık gri varsayılan arka plan
            text_color: Color::rgb(0.0, 0.0, 0.0),        // Siyah varsayılan metin rengi
            on_click: None,
            state: ButtonState::Normal,
        }
    }

    // Tıklama olayını ayarlamak için bir metot
    pub fn set_on_click<F>(&mut self, callback: F)
    where
        F: 'static + FnMut(),
    {
        self.on_click = Some(Box::new(callback));
    }

    // Arka plan rengini ayarlamak için bir metot
    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    // Metin rengini ayarlamak için bir metot
    pub fn set_text_color(&mut self, color: Color) {
        self.text_color = color;
    }

    pub fn handle_event(&mut self, event: &Event) {
        match event {
            Event::Mouse(MouseEvent::Moved { x, y }) => {
                if self.rect.contains(Point::new(*x, *y)) {
                    if self.state == ButtonState::Normal {
                        self.state = ButtonState::Hovered;
                        println!("Button '{}' hovered", self.label);
                    }
                } else {
                    if self.state == ButtonState::Hovered || self.state == ButtonState::Pressed {
                        self.state = ButtonState::Normal;
                        println!("Button '{}' unhovered", self.label);
                    }
                }
            }
            Event::Mouse(MouseEvent::ButtonPressed { button, x, y }) => {
                if *button == MouseButton::Left && self.rect.contains(Point::new(*x, *y)) {
                    self.state = ButtonState::Pressed;
                    println!("Button '{}' pressed", self.label);
                }
            }
            Event::Mouse(MouseEvent::ButtonReleased { button, x, y }) => {
                if *button == MouseButton::Left && self.rect.contains(Point::new(*x, *y)) {
                    if self.state == ButtonState::Pressed {
                        println!("Button '{}' clicked", self.label);
                        // Tıklama olayını tetikle
                        if let Some(callback) = &mut self.on_click {
                            (callback)();
                        }
                        self.state = ButtonState::Hovered; // Tıklamadan sonra hover durumuna geçilebilir
                    }
                } else if self.state == ButtonState::Pressed {
                    self.state = ButtonState::Normal; // Düğme dışarıda bırakıldıysa normal duruma dön
                    println!("Button '{}' press released outside", self.label);
                }
            }
            _ => {}
        }
    }

    // Çizim metodu (Renderer türünü varsayıyoruz)
    pub fn draw<R>(&self, renderer: &mut R)
    where
        R: Renderer, // Renderer trait'ini ileride tanımlayacağız
    {
        let color = match self.state {
            ButtonState::Normal => self.background_color,
            ButtonState::Hovered => Color::rgb(0.9, 0.9, 0.9), // Biraz daha açık gri
            ButtonState::Pressed => Color::rgb(0.7, 0.7, 0.7), // Biraz daha koyu gri
        };
        renderer.draw_rect(self.rect, color);

        // Metni çizme mantığı (basit bir şekilde merkezde varsayıyoruz)
        let text_x = self.rect.x + self.rect.width / 2.0;
        let text_y = self.rect.y + self.rect.height / 2.0;
        renderer.draw_text(&self.label, Point::new(text_x, text_y), self.text_color);
    }
}

// Şu an için basit bir Renderer trait tanımı yapıyoruz.
// Gerçek uygulamada bu trait daha kapsamlı olacaktır.
pub trait Renderer {
    fn draw_rect(&mut self, rect: Rect, color: Color);
    fn draw_text(&mut self, text: &str, position: Point, color: Color);
}