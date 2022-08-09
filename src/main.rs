mod atoms;

fn main() {

   let mut atoms = Vec::<atoms::Atom>::new();

   for e in 1..=28 {
      let a = atoms::Atom::from_pen(e);
      println!("e={}; e.p.s={:?}. e.p.ss={:?}, az.p.ss={:?}", e, a.electron_shell().electrons_per_shell(), a.electron_shell().electrons_per_subshell(), a.electron_shell().azimuthal_quantum_numbers_per_subshell());
   }
   
}
