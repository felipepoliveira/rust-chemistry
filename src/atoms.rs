#[derive(Debug)]
pub struct ElectronShell {
    electrons : u16,
    electrons_per_shell : Vec<u16>,
}

impl ElectronShell {
    pub fn from_electrons(num_of_electrons : u16) -> ElectronShell {
        let mut electrons_per_shell = Vec::<u16>::new();
        let mut electrons : u16 = 0;
         
        
        let mut max_azimuthal_quantum_number : u16 = 0;

        // go to 1s, 2s, 3s, 4s ...
         for n in 0..num_of_electrons {

            if n != 0 && n % 2 == 0 {
                max_azimuthal_quantum_number += 1;
            }

            // iterate from the maximum current value of the azimuthal quantum number to the minimum (0)
            for az in (0..=max_azimuthal_quantum_number).rev() {
                // calculate the maximum amount of electrons in the current azimuthal quantum number
                // me = 4 * (l + 1) - 2;
                let mut max_electrons = 4 * (az + 1) - 2;

                // increase the total amount of counted electrons
                electrons += max_electrons;

                // if the electrons count exceeds the limit, decrease it to reach the max electrons
                if electrons > num_of_electrons {
                    max_electrons -= electrons - num_of_electrons;
                }

                // include the electrons in the shell counter
                electrons_per_shell.push(max_electrons);

                // break if reached limit
                if electrons >= num_of_electrons {
                    break;
                }
            }
            
            // break if reached limit
            if electrons >= num_of_electrons {
                break;
            }
         }

        ElectronShell { electrons: num_of_electrons, electrons_per_shell }
        
    }
}

pub struct Atom {
    symbol : String,
    protons : u16,
    electrons : u16,
    weight : f64,
}

/// Store the Subshell labels used to describe an Atom electron configuration
pub static SUBSHELL_LABELS : &'static [char] = &['s', 'p', 'd', 'f', 'g'];

impl Atom {

    pub fn new(symbol : String, protons: u16, weight: f64) -> Atom {
        Atom {
            symbol : symbol,
            protons : protons,
            weight : weight,
            electrons : protons,
        }
    }

    pub fn H() -> Atom {
        Atom::new(String::from("H"), 1, 1.008)
    }
}