const A4:f32 = 440.00;

pub struct Octave {
    pub base: u8,
    pub freqs: Vec<f32>,
}

impl Octave {
    pub fn new(b: u8) -> Self {
        let a = b;
        Octave{
            base: a,
            freqs: get_freqs(a),
        }
    }
}

impl Drop for Octave{
    fn drop(&mut self) {
        println!("The frequencies for octave {0} from C{0} to B{0} are:", self.base);
        for i in &self.freqs {
            let temp = format!("{:.2}Hz", i);
            print!("{:^10}", temp);
        }
        println!("\n");
    }
}

fn get_freqs(x: u8) -> Vec<f32> {
    let mut v:Vec<f32> = vec![];
    let f = A4 * 2.0_f32.powf((x as f32) - 4.0_f32);
    for i in 0..12 {
        let temp:f32 = f * 2.0_f32.powf((i - 9) as f32/12.0_f32);
        v.push(temp);
    }
    v
}

