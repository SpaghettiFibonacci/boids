const SPEED_LIMIT: f32 = 1.8;
const VISUAL_RANGE: f32 = 33.0;
const MIN_DISTANCE: f32 = 7.0;

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
            let distance = ((bird.x - self.x).powi(2) + (bird.y - self.y).powi(2)).sqrt();
            if distance < VISUAL_RANGE {
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
            let distance = ((bird.x - self.x).powi(2) + (bird.y - self.y).powi(2)).sqrt();
            if distance < VISUAL_RANGE {
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
        // separation is more powerful when they're clumped together
        let mut x = 0.0;
        let mut y = 0.0;
        let mut count = 0;
        for bird in birds {
            let distance = ((bird.x - self.x).powi(2) + (bird.y - self.y).powi(2)).sqrt();
            if distance < MIN_DISTANCE {
                x += (self.x - bird.x) * 3.5;
                y += (self.y - bird.y) * 3.5;
                count += 1;
            }
        }
        if count > 0 {
            (x / count as f32, y / count as f32)
        } else {
            (0.0, 0.0)
        }
    }
    pub fn run(&mut self, birds: &Vec<Bird>, target: (f32, f32), width: f32, height: f32) {
        let birds_in_visual_range = birds
            .iter()
            .filter(|bird| {
                let distance = ((bird.x - self.x).powi(2) + (bird.y - self.y).powi(2)).sqrt();
                distance < VISUAL_RANGE
            })
            .collect::<Vec<&Bird>>();

        let (ax, ay) = self.alignment(&birds_in_visual_range);
        let (cx, cy) = self.cohesion(&birds_in_visual_range);
        let (sx, sy) = self.separation(&birds_in_visual_range);
        let (tx, ty) = target;
        self.dx += ax * 0.02 + cx * 0.01 + sx * 0.02 + (tx - self.x) * 0.01;
        self.dy += ay * 0.02 + cy * 0.01 + sy * 0.02 + (ty - self.y) * 0.01;

        let speed = (self.dx.powi(2) + self.dy.powi(2)).sqrt();
        if speed > SPEED_LIMIT && sx == 0.0 && sy == 0.0 {
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
