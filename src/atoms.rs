#[derive(Debug)]
pub struct ElectronShell {
    electrons : u16,
    electrons_per_shell : Vec<u16>,
    azimuthal_quantum_numbers_per_shell : Vec<u8>,
}

impl ElectronShell {
    pub fn from_electrons(num_of_electrons : u16) -> ElectronShell {
        if num_of_electrons > 60000 {
            panic!("The maximum amount of electrons that can be calculated is 60000. {} given", num_of_electrons)
        }

        let mut electrons_per_shell = Vec::<u16>::new();
        let mut electrons : u16 = 0;
        let mut azimuthal_quantum_numbers_per_shell : Vec<u8> = Vec::new();
         
        
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

                // store the current azimuthal quantum number
                azimuthal_quantum_numbers_per_shell.push(az as u8);

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

        ElectronShell { electrons: num_of_electrons, electrons_per_shell, azimuthal_quantum_numbers_per_shell }
        
    }

    /// Return the azimuthal quantum number on this the electron shell in order from the closest to the nucleus to the farthest
    pub fn azimuthal_quantum_numbers_per_shell(&self) -> &Vec<u8> {
        &self.azimuthal_quantum_numbers_per_shell
    }

    /// Return the total amount of electrons from this electron shell
    pub fn total_electrons(&self) -> u16 {
        self.electrons
    }

    /// Return the amount of electrons per shell in order from the closest to the nucleus to the farthest
    pub fn electrons_per_shell(&self) -> &Vec<u16> {
        &self.electrons_per_shell
    }

    /// Return an string representation from the shells
    pub fn shell_to_string(&self) -> String {
        self.electrons_per_shell
            .iter().zip(self.azimuthal_quantum_numbers_per_shell.iter())
            .map(|(e_value, e_az)| {
                format!("{}{} ", e_value, subshell_label_from_az_value(*e_az as usize))
            })
            .collect()
    }   

}

pub struct Atom {
    protons : u16,
    neutrons : u16,
    electrons : u16,
    weight : f64,
    electron_shell: ElectronShell,
}

/// Store the Subshell labels used to describe an Atom electron configuration
pub static SUBSHELL_LABELS : &'static [char] = &['s', 'p', 'd', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm'];

fn subshell_label_from_az_value(az: usize) -> String {
    if az < SUBSHELL_LABELS.len() {
        return format!("{}", SUBSHELL_LABELS[az]);
    }
    else {
        return format!("n{}", (az - SUBSHELL_LABELS.len() + 1));
    }
}

impl Atom {

    pub fn from_pe_n(protons_and_electrons: u16, neutrons : u16) -> Atom {
        Atom {
            protons : protons_and_electrons,
            neutrons,
            weight : (protons_and_electrons + neutrons) as f64,
            electrons : protons_and_electrons,
            electron_shell : ElectronShell::from_electrons(protons_and_electrons)
        }
    }

    pub fn from_p_e_n(protons : u16, electrons: u16, neutrons: u16) -> Atom {
        Atom {
            protons,
            neutrons,
            weight : (protons + neutrons) as f64,
            electrons,
            electron_shell : ElectronShell::from_electrons(electrons)
        }
    }

    /// return electron shell data from this atom
    pub fn electron_shell(&self) -> &ElectronShell {
        &self.electron_shell
    }

    /// Return the total amount of protons from this atom
    pub fn protons(&self) -> u16 {
        self.protons
    }

    /// Return the total amount of electrons from this atom
    pub fn electrons(&self) -> u16 {
        self.electrons
    }

    /// Return the total amount of neutrons from this atom
    pub fn neutrons(&self) -> u16 {
        self.neutrons
    }

    /// Return the atomic mass (weight) of this atom
    pub fn atomic_mass(&self) -> f64 {
        self.weight
    }

    /// Return the eletric charge of this atom (p - e)
    pub fn electric_charge(&self) -> i16 {
        (self.protons as i32 - self.electrons as i32) as i16
    }

    pub fn is_isotope_of(&self, other : &Atom) -> bool {
        (self.protons == other.protons)
    }
}