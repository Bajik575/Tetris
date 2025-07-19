use std::io;
use console::{Key, Term};
use rand::Rng;
use rand::rngs;
use std::sync::mpsc;
use std::io::Write;

const HEIGHT: usize = 20;
const WIDTH: usize = 10;
#[derive(Clone)]
struct Point(u8, u8);
#[derive(Clone)]
struct Figure {
    points: [Point;5],
    rng : rngs::ThreadRng,
}
impl Figure{

    fn new() -> Self{
        let mut rng = rand::thread_rng();
        let num: u8 = rng.gen_range(0..8);
        Self{points:[
            Point(num + 0,0),
            Point(num + 1,0),
            Point(num + 2,0),
            Point(num + 1,1),
            Point(num + 1,2)],
            rng}
    }
}
fn update<'a>(key: Key, arr:& mut [[Option<&'a Figure>; WIDTH]; HEIGHT], is_spawn:&mut bool, figures: &'a mut Vec<Figure>){
    if *is_spawn {
        let fig = Figure::new();
        figures.push(fig);
        let fig: &'a Figure = figures.last().unwrap();
        for i in figures.last().unwrap().points.iter(){
            arr[i.1 as usize][i.0 as usize] = Some(fig);
        }
        *is_spawn = false;
    }
    for h in 0..HEIGHT {
        for w in 0..WIDTH {

        }
    }
}
fn display(arr:&[[Option<& Figure>; WIDTH]; HEIGHT]) {
    for h in 0..HEIGHT {
        for w in 0..WIDTH {
            if arr[h][w].is_none() { print!("0") }
            else                  { print!("@") }
        }
        print!("\n");
    }

}
fn main() {
    loop {
        let (sender, reader) = mpsc::channel::<Key>();
        std::thread::spawn(move || {
            let mut key: Key;
            loop {
                key = Term::stdout().read_key().unwrap().into();
                match sender.send(key) {
                    Err(err) => panic!("{err}"),
                    _ => (),
                };
            }
        });
        let mut arr:[[Option<&Figure>; WIDTH]; HEIGHT] = [[None; WIDTH]; HEIGHT];
        let mut is_spawn = true;
        let mut figures: Vec<Figure> = vec![];
        loop {
            update(reader.recv_timeout(std::time::Duration::from_millis(400)).unwrap(),&mut arr,&mut is_spawn,&mut figures);
            print!("\n\n\n\n\n");
            display(&arr);
        }
    }
}