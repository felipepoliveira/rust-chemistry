mod atoms;

use atoms::Atom;

fn main() {
    println!("{:?}", atoms::ElectronShell::from_electrons(17));
}
