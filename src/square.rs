#[derive(Debug)]
struct Vec2(f64, f64);

pub struct Square {
    pub size: f64,
    pub position: (f64, f64),
    pub mass: f64,
    forces: Vec<Vec2>,
    velocity: Vec2,
    acceleration: Vec2,
}

impl Square {
    pub fn new(size: f64, position: (f64, f64), mass: f64) -> Self {
        Self {
            size,
            position,
            mass,
            forces: Vec::new(),
            velocity: Vec2(10.0, -5.0),
            acceleration: Vec2(0.0, 0.0),
        }
    }
    fn force(&mut self, x: f64, y: f64) {
        self.forces.push(Vec2(x, y));
    }
    fn evaluate_acceleration(&mut self) {
        let forces = std::mem::take(&mut self.forces);
        println!("f: {:?}", forces);
        let acceleration = forces
            .into_iter()
            .fold(Vec2(0.0, 0.0), |accumulation, current| {
                Vec2(accumulation.0 + current.0, accumulation.1 + current.1)
            });
        self.acceleration.0 += acceleration.0 / self.mass;
        self.acceleration.1 += acceleration.1 / self.mass;
    }

    fn evaluate_velocity(&mut self, delta: f64) {
        self.velocity.0 += self.acceleration.0 * delta;
        self.velocity.1 += self.acceleration.1 * delta;
    }

    fn evaluate_position(&mut self, delta: f64) {
        self.position.0 += self.velocity.0 * delta;
        self.position.1 += self.velocity.1 * delta;
    }

    pub fn evaluate(&mut self, delta: f64) {
        self.acceleration = Vec2(0.0, 0.0);
        let drag_x = self.calculate_drag(self.velocity.0);
        let drag_y = self.calculate_drag(self.velocity.1);
        self.force(drag_x, drag_y);

        let prev_speed = self.velocity.1;

        self.evaluate_acceleration();
        self.evaluate_velocity(delta);
        self.evaluate_position(delta);

        println!("sD: {}", self.velocity.1 - prev_speed);
        println!("d: {drag_x}, {drag_y}");
        println!("a: {:?}", self.acceleration);
        println!("s: {:?}\n", self.velocity);
    }

    pub fn propel(&mut self, x: f64, y: f64) {
        self.force(x, y);
    }

    fn calculate_drag(&self, speed: f64) -> f64 {
        let naive_density = 0.0012;
        let drag_coefficient = 1.05;
        let force = 0.5 * speed * speed * self.size * naive_density * drag_coefficient;
        if speed > 0.0 {
            -force
        } else {
            force
        }
    }
}
