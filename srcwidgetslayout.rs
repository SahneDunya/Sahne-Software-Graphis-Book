use crate::core::graphics::Rect;
use crate::widgets::button::Button; // Örnek olarak Button'ı kullandık
use crate::widgets::label::Label;    // Örnek olarak Label'ı kullandık

// Layout'a eklenebilecek öğeler için bir trait (Button, Label vb.)
pub trait LayoutItem {
    fn set_rect(&mut self, rect: Rect);
    fn get_rect(&self) -> Rect;
    // İlerleyen aşamalarda çizim ve olay işleme metotları eklenebilir.
    // fn draw(&self, renderer: &mut /* Renderer türünüz */);
    // fn handle_event(&mut self, event: &crate::core::event::Event);
}

// LayoutItem trait'ini Button ve Label için implemente edelim
impl LayoutItem for Button {
    fn set_rect(&mut self, rect: Rect) {
        self.rect = rect;
    }

    fn get_rect(&self) -> Rect {
        self.rect
    }
}

impl LayoutItem for Label {
    fn set_rect(&mut self, rect: Rect) {
        self.rect = rect;
    }

    fn get_rect(&self) -> Rect {
        self.rect
    }
}

#[derive(Debug)]
pub struct VBox {
    pub rect: Rect,
    pub children: Vec<Box<dyn LayoutItem>>, // LayoutItem trait'i ile farklı widget türlerini tutabiliriz
    pub spacing: f32,
}

impl VBox {
    pub fn new(rect: Rect) -> Self {
        VBox {
            rect,
            children: Vec::new(),
            spacing: 0.0,
        }
    }

    pub fn add_child<T: 'static + LayoutItem>(&mut self, child: T) {
        self.children.push(Box::new(child));
    }

    pub fn set_spacing(&mut self, spacing: f32) {
        self.spacing = spacing;
    }

    pub fn layout(&mut self, available_rect: Rect) {
        self.rect = available_rect; // VBox'ın kendi rect'ini de güncelleyelim
        let mut current_y = available_rect.y;
        let total_height = available_rect.height;
        let num_children = self.children.len() as f32;
        let spacing_total = self.spacing * (num_children - 1.0).max(0.0);
        let available_height_per_child = (total_height - spacing_total) / num_children.max(1.0);

        for child in &mut self.children {
            let child_height = available_height_per_child; // Basit bir eşit dağılım
            child.set_rect(Rect::new(available_rect.x, current_y, available_rect.width, child_height));
            current_y += child_height + self.spacing;
        }
    }
}

#[derive(Debug)]
pub struct HBox {
    pub rect: Rect,
    pub children: Vec<Box<dyn LayoutItem>>, // LayoutItem trait'i ile farklı widget türlerini tutabiliriz
    pub spacing: f32,
}

impl HBox {
    pub fn new(rect: Rect) -> Self {
        HBox {
            rect,
            children: Vec::new(),
            spacing: 0.0,
        }
    }

    pub fn add_child<T: 'static + LayoutItem>(&mut self, child: T) {
        self.children.push(Box::new(child));
    }

    pub fn set_spacing(&mut self, spacing: f32) {
        self.spacing = spacing;
    }

    pub fn layout(&mut self, available_rect: Rect) {
        self.rect = available_rect; // HBox'ın kendi rect'ini de güncelleyelim
        let mut current_x = available_rect.x;
        let total_width = available_rect.width;
        let num_children = self.children.len() as f32;
        let spacing_total = self.spacing * (num_children - 1.0).max(0.0);
        let available_width_per_child = (total_width - spacing_total) / num_children.max(1.0);

        for child in &mut self.children {
            let child_width = available_width_per_child; // Basit bir eşit dağılım
            child.set_rect(Rect::new(current_x, available_rect.y, child_width, available_rect.height));
            current_x += child_width + self.spacing;
        }
    }
}