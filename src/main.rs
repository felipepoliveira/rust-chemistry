mod atoms;

fn main() {
   let h1 = atoms::Atom::from_p_e_n(1, 2, 0);
   let h2 = atoms::Atom::from_pe_n(1, 1);

   println!("H-1 a.m: {}", h1.atomic_mass());
   println!("H-1 electric charge: {}", h1.electric_charge());
   println!("H-2 a.m: {}", h2.atomic_mass());
   
   println!("H-1 is isotope of H-2: {}", h1.is_isotope_of(&h2));
}
