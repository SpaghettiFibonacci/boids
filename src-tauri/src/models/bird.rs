const SPEED_LIMIT: f32 = 4.0;
const VISUAL_RANGE: f32 = 52.0;
const MIN_DISTANCE: f32 = 34.0;

#[derive(Debug, Clone, Copy)]
pub struct Bird {
    pub x: f32,
    pub y: f32,
    pub dx: f32,
    pub dy: f32,
}

impl Bird {
    pub fn new(x: f32, y: f32, dx: f32, dy: f32) -> Self {
        Bird { x, y, dx, dy }
    }
    fn alignment(&self, birds: &Vec<&Bird>) -> (f32, f32) {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut count = 0;
        for bird in birds {
            if (bird.x - self.x).abs() < VISUAL_RANGE && (bird.y - self.y).abs() < VISUAL_RANGE {
                // use alignment weight
                x += bird.dx;
                y += bird.dy;
                count += 1;
            }
        }
        if count > 0 {
            (x / count as f32, y / count as f32)
        } else {
            (0.0, 0.0)
        }
    }
    fn cohesion(&self, birds: &Vec<&Bird>) -> (f32, f32) {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut count = 0;
        for bird in birds {
            if (bird.x - self.x).abs() < VISUAL_RANGE && (bird.y - self.y).abs() < VISUAL_RANGE {
                x += bird.x;
                y += bird.y;
                count += 1;
            }
        }
        if count > 0 {
            (x / count as f32, y / count as f32)
        } else {
            (0.0, 0.0)
        }
    }
    fn separation(&self, birds: &Vec<&Bird>) -> (f32, f32) {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut count = 0;
        for bird in birds {
            if (bird.x - self.x).abs() < VISUAL_RANGE && (bird.y - self.y).abs() < VISUAL_RANGE {
                if (bird.x - self.x).abs() < MIN_DISTANCE && (bird.y - self.y).abs() < MIN_DISTANCE
                {
                    x += self.x - bird.x;
                    y += self.y - bird.y;
                    count += 1;
                }
            }
        }
        if count > 0 {
            (x / count as f32, y / count as f32)
        } else {
            (0.0, 0.0)
        }
    }
    pub fn run(&mut self, birds: &Vec<Bird>, target: (f32, f32), width: f32, height: f32) {
        let vec_birds_close = birds
            .iter()
            .filter(|bird| {
                (bird.x - self.x).abs() < VISUAL_RANGE && (bird.y - self.y).abs() < VISUAL_RANGE
            })
            .collect::<Vec<&Bird>>();

        // run but when out of bounds continue with velocity instead of hugging wall
        let (mut ax, mut ay) = self.alignment(&vec_birds_close);
        ax += 0.1;
        ay += 0.1;
        let (cx, cy) = self.cohesion(&vec_birds_close);
        let (sx, sy) = self.separation(&vec_birds_close);
        let (tx, ty) = target;
        self.dx += ax * 0.01 + cx * 0.01 + sx * 0.01 + (tx - self.x) * 0.01;
        self.dy += ay * 0.01 + cy * 0.01 + sy * 0.01 + (ty - self.y) * 0.01;
        let speed = (self.dx * self.dx + self.dy * self.dy).sqrt();
        if speed > SPEED_LIMIT {
            self.dx = self.dx / speed * SPEED_LIMIT;
            self.dy = self.dy / speed * SPEED_LIMIT;
        }
        self.x += self.dx;
        self.y += self.dy;
        if self.x < 0.0 {
            self.x = 0.0;
            self.dx = -self.dx;
        }
        if self.x > width {
            self.x = width;
            self.dx = -self.dx;
        }
        if self.y < 0.0 {
            self.y = 0.0;
            self.dy = -self.dy;
        }
        if self.y > height {
            self.y = height;
            self.dy = -self.dy;
        }
    }
}
