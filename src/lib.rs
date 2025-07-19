use rand::Rng;
use rand::prelude::ThreadRng;

pub mod constants;

// Point
#[derive(Clone, Copy)]
pub struct Point(pub i8, pub i8);
impl std::ops::Add for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(rhs.0 + self.0, rhs.1 + self.1)
    }
}
impl FromIterator<Point> for [Point; constants::POINTS_IN_FIGURE] {
    fn from_iter<T: IntoIterator<Item = Point>>(iter: T) -> Self {
        let mut iter = iter.into_iter();
        let mut arr: [Point; constants::POINTS_IN_FIGURE] =
            [Point(0, 0); constants::POINTS_IN_FIGURE];
        for point in arr.iter_mut() {
            *point = iter.next().unwrap();
        }
        arr
    }
}

// RotateState(for figure)
#[derive(Copy, Clone)]
enum RotateState {
    Deg0,
    Deg1,
    Deg2,
    Deg3,
}
impl RotateState {
    #[allow(dead_code)]
    fn get_state_id(&self) -> usize {
        match self {
            RotateState::Deg0 => 0,
            RotateState::Deg1 => 1,
            RotateState::Deg2 => 2,
            RotateState::Deg3 => 3,
        }
    }
    fn next(&mut self) -> usize {
        let num;
        (*self, num) = match self {
            RotateState::Deg0 => (RotateState::Deg1, 1),
            RotateState::Deg1 => (RotateState::Deg2, 2),
            RotateState::Deg2 => (RotateState::Deg3, 3),
            RotateState::Deg3 => (RotateState::Deg0, 0),
        };
        num
    }
}

// Figure
type OutputArr = [[bool; constants::WIDTH]; constants::HEIGHT];
#[derive(Copy, Clone)]
pub struct Figure {
    pub main_point: Point,
    pub points: [Point; constants::POINTS_IN_FIGURE],
    rotate_state: RotateState,
    type_id: u8,
}
impl Figure {
    pub fn new() -> Self {
        let mut rng: ThreadRng = rand::rng();
        let rotate_state = match rng.random_range(0..4) {
            0 => RotateState::Deg0,
            1 => RotateState::Deg1,
            2 => RotateState::Deg2,
            3 => RotateState::Deg3,
            _ => panic!("num>3"),
        };
        let type_id = rng.random_range(0..4);
        let shift = rng.random_range(0..7);
        Self {
            main_point: Point(shift, 0),
            points: constants::FIGURES_TYPES[rotate_state.clone() as usize][type_id as usize]
                .clone()
                .iter()
                .map(|p| Point(p.0 + shift, p.1))
                .collect(),
            rotate_state,
            type_id,
        }
    }
    pub fn rotate(&mut self) -> &Self {
        let rotate_state_id = self.rotate_state.next();
        let type_id = self.type_id;
        for (i, point) in self.points.iter_mut().enumerate() {
            *point =
                self.main_point + constants::FIGURES_TYPES[rotate_state_id][type_id as usize][i];
        }
        self
    }
    pub fn is_valid(&self, arr: &OutputArr) -> bool {
        for point in self.points.iter() {
            if  point.0 > constants::WIDTH  as i8 - 1 ||
                point.0 < 0 ||
                point.1 > constants::HEIGHT as i8 - 1{
                return false;
            }
            if arr[point.1 as usize][point.0 as usize]{
                return false;
            }

        }
        true
    }
    pub fn hide(&self, arr: &mut OutputArr) {
        for point in self.points.iter() {
            arr[point.1 as usize][point.0 as usize] = false;
        }
    }
    pub fn show(&self, arr: &mut OutputArr) {
        for point in self.points.iter() {
            arr[point.1 as usize][point.0 as usize] = true;
        }
    }
    pub fn falldown(&mut self) -> &Self {
        for point in self.points.iter_mut() {
            point.1 += 1;
        }
        self.main_point.1 += 1;
        self
    }
}
