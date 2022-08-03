mod atoms;

fn main() {

   let ne = atoms::Atom::from_pen(108);
   println!("{:?}", ne.electron_shell());
   
}
