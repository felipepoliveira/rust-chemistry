#[derive(Debug)]
pub enum ChemicalSet {
    AlkaliMetal,
    AlkaliEarthMetal,
    Chalcogen,
    Halogen,
    NobleGas,
}

impl ChemicalSet {
    /// Calculate and return the chemical sets of an element depending on its electrons
    pub fn from_valence_electrons(electrons_in_valence_orbital: u16, az_quantum_number_in_valence_orbital: u16) -> Vec<ChemicalSet> {
        // calculate the chemical set based on the value from the electrons on the valence
        let chemical_set_from_valence_electrons =  match (az_quantum_number_in_valence_orbital, electrons_in_valence_orbital){
            // l=0, h.l=s, e=1
            (0, 1) => Some(ChemicalSet::AlkaliMetal),
            // l=0, h.l=s, e=2
            (0, 2) => Some(ChemicalSet::AlkaliEarthMetal),
            // l=1, h.l=p, e=4
            (1, 4) => Some(ChemicalSet::Chalcogen),
            // l=1, h.l=p, e=5
            (1, 5) => Some(ChemicalSet::Halogen),
            // l=1, h.l=p, e=6
            (1, 6) => Some(ChemicalSet::NobleGas),
            _ => None,
        };

        // the vector that will be returned
        let mut chemical_sets = Vec::<ChemicalSet>::new();

        // store the chemical set if returned some
        if let Some(chemical_set) = chemical_set_from_valence_electrons {
            chemical_sets.push(chemical_set);
        }
        
        return chemical_sets
    }
}

#[derive(Debug)]
pub struct ElectronShell {
    electrons : u16,
    electrons_per_shell : Vec<u16>,
    electrons_per_subshell : Vec<u16>,
    azimuthal_quantum_numbers_per_subshell : Vec<u8>,
    //chemical_sets : Vec<ChemicalSet>,
}

impl ElectronShell {
    pub fn from_electrons(num_of_electrons : u16) -> ElectronShell {
        if num_of_electrons > 60000 {
            panic!("The maximum amount of electrons that can be calculated is 60000. {} given", num_of_electrons)
        }

        // store total electrons
        let mut electrons : u16 = 0;

        // store shell data
        let mut electrons_per_shell = Vec::<u16>::new();

        // store subshell data
        let mut electrons_per_subshell = Vec::<u16>::new();
        let mut azimuthal_quantum_numbers_per_subshell : Vec<u8> = Vec::new();
        
        // store the maximum azimuthal quantum number
        let mut max_azimuthal_quantum_number : u16 = 0;

        let mut max_orbits_per_shell : u16 = 1;
        let mut current_orbits_per_shell: u16 = 0;
        let mut electrons_in_current_shell :  u16 = 2;
        let mut electron_per_shell_sum : u16 = 0;

        // go to 1s, 2s, 3s, 4s ...
         for n_shell_loop in 0..num_of_electrons {

            // for each 2 'n' orbit increase the azimuthal quantum number + 1
            if n_shell_loop != 0 && n_shell_loop % 2 == 0 {
                max_azimuthal_quantum_number += 1;
            }

            // calculate subshell values
            for az in (0..=max_azimuthal_quantum_number).rev() {
                // calculate the maximum amount of electrons in the current azimuthal quantum number
                // me = 4 * (l + 1) - 2;
                let mut max_electrons_in_subshell = 4 * (az + 1) - 2;

                // store the current azimuthal quantum number
                azimuthal_quantum_numbers_per_subshell.push(az as u8);

                // increase the total amount of counted electrons
                electrons += max_electrons_in_subshell;

                // if the electrons count exceeds the limit, decrease it to reach the max electrons
                if electrons > num_of_electrons {
                    max_electrons_in_subshell -= electrons - num_of_electrons;
                }

                // include the electrons in the shell counter
                electrons_per_subshell.push(max_electrons_in_subshell);

                // increase the number of electrons per shell and add an orbit in the counter
                current_orbits_per_shell += 1;
                
                // check if the current orbit reached the max orbits per shell limit or 
                // reached the limit of electrons in the electrons shell
                if current_orbits_per_shell == max_orbits_per_shell && electron_per_shell_sum < num_of_electrons{

                    electron_per_shell_sum += electrons_in_current_shell;

                    
                    // add the total number of electrons in thell
                    if electron_per_shell_sum <=  num_of_electrons {
                        electrons_per_shell.push(electrons_in_current_shell);
                    }
                    else {
                        electrons_per_shell.push(electrons_in_current_shell - (electron_per_shell_sum - num_of_electrons));
                    }
                    

                    // calculate next value
                    electrons_in_current_shell = 2 + electrons_in_current_shell + 4 * max_orbits_per_shell;

                    max_orbits_per_shell += 1;
                    current_orbits_per_shell = 0;
                }

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

         // add if remaining
         if electron_per_shell_sum < num_of_electrons {
            electrons_per_shell.push(num_of_electrons - electron_per_shell_sum);
         }

        //  while *electrons_per_shell.last().unwrap() > 8 {
        //     let last_electron_shell = *electrons_per_shell.last().unwrap();
        //     electrons_per_shell.pop();
        //     electrons_per_shell.push(8);
        //     electrons_per_shell.push(last_electron_shell - 8);
        //  }

        ElectronShell { 
            electrons: num_of_electrons, 
            electrons_per_shell,
            electrons_per_subshell, 
            azimuthal_quantum_numbers_per_subshell, 
            //chemical_sets : ChemicalSet::from_valence_electrons(valence_electrons, valence_az) 
        }
    }

    /// Return the azimuthal quantum number on this the electron shell in order from the closest to the nucleus to the farthest
    pub fn azimuthal_quantum_numbers_per_subshell(&self) -> &Vec<u8> {
        &self.azimuthal_quantum_numbers_per_subshell
    }

    /// Return the total amount of electrons from this electron shell
    pub fn total_electrons(&self) -> u16 {
        self.electrons
    }

    /// Return the amount of electrons per shell in order from the closest to the nucleus to the farthest
    pub fn electrons_per_subshell(&self) -> &Vec<u16> {
        &self.electrons_per_subshell
    }

    /// Return an string representation from the shells
    pub fn subshell_to_string(&self) -> String {
        self.electrons_per_subshell
            .iter().zip(self.azimuthal_quantum_numbers_per_subshell.iter())
            .map(|(e_value, e_az)| {
                format!("{}{} ", e_value, subshell_label_from_az_value(*e_az as usize))
            })
            .collect()
    }

    // pub fn chemical_sets(&self) -> &Vec<ChemicalSet> {
    //     &self.chemical_sets
    // }

}

#[derive(Debug)]
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

    pub fn from_p_e_n(protons : u16, electrons: u16, neutrons: u16) -> Atom {
        Atom {
            protons,
            neutrons,
            weight : (protons + neutrons) as f64,
            electrons,
            electron_shell : ElectronShell::from_electrons(electrons)
        }
    }

    pub fn from_pe_n(protons_and_electrons: u16, neutrons : u16) -> Atom {
        Atom::from_p_e_n(protons_and_electrons, protons_and_electrons, neutrons)
    }

    pub fn from_pen(protons_electrons_and_neutrons : u16) -> Atom {
        Atom::from_pe_n(protons_electrons_and_neutrons, protons_electrons_and_neutrons)
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
    pub fn ion_electric_charge(&self) -> i16 {
        (self.protons as i32 - self.electrons as i32) as i16
    }

    /// Return an flag indicating if this atom is a isotope of another given atom reference
    pub fn is_isotope_of(&self, other : &Atom) -> bool {
        self.protons == other.protons
    }
}