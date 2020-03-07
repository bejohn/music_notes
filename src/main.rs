use music_notes::Octave;

#[allow(unused_variables)]
fn main(){

    let mut v:Vec<Octave> = vec![];
    print!("Octaves are from frequencies \nC  C#  D D#  E  F  F#  G  G#  ");
    print!("A  A#  B\n\n");

    for i in (0..=5).rev() {
        let temp = Octave::new(i);
        v.push(temp);
    }
}
