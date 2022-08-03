mod atoms;

fn main() {

   let mut atoms = Vec::<atoms::Atom>::new();

   for e in 18..=26 {
      println!("{:?}", atoms::Atom::from_pen(e).electron_shell());
   }
   
}
