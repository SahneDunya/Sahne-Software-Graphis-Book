#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Point { x, y }
    }

    // Yeni özellikler: Vektör işlemleri
    pub fn add(&self, other: Point) -> Self {
        Point { x: self.x + other.x, y: self.y + other.y }
    }

    pub fn subtract(&self, other: Point) -> Self {
        Point { x: self.x - other.x, y: self.y - other.y }
    }

    // Yeni özellik: İki nokta arasındaki mesafeyi hesaplama
    pub fn distance(&self, other: Point) -> f32 {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        (dx * dx + dy * dy).sqrt()
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rect {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Rect { x, y, width, height }
    }

    pub fn contains(&self, point: Point) -> bool {
        point.x >= self.x && point.x <= self.x + self.width &&
        point.y >= self.y && point.y <= self.y + self.height
    }

    // Yeni özellik: Dikdörtgenin merkez noktasını bulma
    pub fn center(&self) -> Point {
        Point::new(self.x + self.width / 2.0, self.y + self.height / 2.0)
    }

    // Yeni özellik: Başka bir dikdörtgenle kesişip kesişmediğini kontrol etme
    pub fn intersects(&self, other: &Rect) -> bool {
        self.x < other.x + other.width &&
        self.x + self.width > other.x &&
        self.y < other.y + other.height &&
        self.y + self.height > other.y
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Line {
    pub start: Point,
    pub end: Point,
}

impl Line {
    pub fn new(start: Point, end: Point) -> Self {
        Line { start, end }
    }

    // Yeni özellik: Çizginin uzunluğunu hesaplama
    pub fn length(&self) -> f32 {
        self.start.distance(self.end)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Circle {
    pub center: Point,
    pub radius: f32,
}

impl Circle {
    pub fn new(center: Point, radius: f32) -> Self {
        Circle { center, radius }
    }

    // Yeni özellik: Dairenin alanını hesaplama
    pub fn area(&self) -> f32 {
        std::f32::consts::PI * self.radius * self.radius
    }
}