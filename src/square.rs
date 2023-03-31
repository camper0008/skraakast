#[derive(Debug)]
struct Vec2(f64, f64);

pub struct Square {
    pub size: f64,
    pub position: (f64, f64),
    pub mass: f64,
    forces: Vec<Vec2>,
    speed: Vec2,
    acceleration: Vec2,
}

impl Square {
    pub fn new(size: f64, position: (f64, f64), mass: f64) -> Self {
        Self {
            size,
            position,
            mass,
            forces: Vec::new(),
            speed: Vec2(0.0, 0.0),
            acceleration: Vec2(0.0, 0.0),
        }
    }
    fn force(&mut self, x: f64, y: f64) {
        self.forces.push(Vec2(x, y));
    }
    fn evaluate_forces(&mut self) {
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

    fn evaluate_acceleration(&mut self, delta: f64) {
        self.speed.0 += self.acceleration.0 * delta;
        self.speed.1 += self.acceleration.1 * delta;
    }

    fn evaluate_speed(&mut self, delta: f64) {
        self.position.0 += self.speed.0 * delta;
        self.position.1 += self.speed.1 * delta;
    }

    pub fn evaluate(&mut self, delta: f64) {
        let drag_x = self.calculate_drag(self.speed.0);
        let drag_y = self.calculate_drag(self.speed.1);
        self.force(drag_x, drag_y);

        println!("\nd: {drag_x}");
        println!("a: {:?}", self.acceleration.0);
        println!("s: {:?}", self.speed.0);
        self.evaluate_forces();
        self.evaluate_acceleration(delta);
        self.evaluate_speed(delta);
    }

    pub fn propel(&mut self, x: f64, y: f64) {
        self.force(x, y);
        //self.force(0.0, 9.82);
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
