mod atoms;

fn main() {

   let intervals = vec![
      54..=70,
      //89..=102,
   ];

   let atomic_level = 103;
   for interval in intervals {
      for e in interval {
         let a = atoms::Atom::from_pen(e);
         println!("{} => {:?}", e, a.electron_shell().electron_subshell_to_string());
      }
   }
   
   
}
