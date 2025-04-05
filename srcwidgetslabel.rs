use crate::core::graphics::{Color, Point, Rect};
use crate::widgets::button::Renderer; // Renderer trait'ini Button'dan alıyoruz

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TextAlignment {
    Left,
    Center,
    Right,
}

#[derive(Debug)]
pub struct Label {
    pub rect: Rect,
    pub text: String,
    pub text_color: Color,
    pub alignment: TextAlignment,
    // İsteğe bağlı: Yazı tipi bilgisi
    // pub font: Font,
}

impl Label {
    pub fn new(rect: Rect, text: String) -> Self {
        Label {
            rect,
            text,
            text_color: Color::rgb(0.0, 0.0, 0.0), // Siyah varsayılan metin rengi
            alignment: TextAlignment::Left,       // Sol hizalama varsayılan
            // font: default_font(),
        }
    }

    pub fn set_text(&mut self, text: String) {
        self.text = text;
    }

    pub fn set_text_color(&mut self, color: Color) {
        self.text_color = color;
    }

    pub fn set_alignment(&mut self, alignment: TextAlignment) {
        self.alignment = alignment;
    }

    pub fn draw<R>(&self, renderer: &mut R)
    where
        R: Renderer,
    {
        let text_x = match self.alignment {
            TextAlignment::Left => self.rect.x,
            TextAlignment::Center => self.rect.x + self.rect.width / 2.0,
            TextAlignment::Right => self.rect.x + self.rect.width,
        };
        let text_y = self.rect.y + self.rect.height / 2.0; // Basitçe dikeyde ortalıyoruz

        renderer.draw_text(&self.text, Point::new(text_x, text_y), self.text_color);
    }
}