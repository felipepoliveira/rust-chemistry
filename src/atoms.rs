use std::collections::HashMap;

#[derive(Debug)]
pub struct ElectronSubshell {
    level : u8,
    shell : u8,
    electrons : u8,
}

impl ElectronSubshell {
    pub fn from_lse(level : u8, shell : u8, electrons : u8) -> ElectronSubshell {
        ElectronSubshell { level, shell, electrons }
    }

    pub fn to_string(&self) -> String {
        format!("{}{}{} ", (self.level), subshell_label_from_az_value(self.shell as usize), self.electrons)
    }
}

#[derive(Debug)]
pub struct ElectronShell {
    electrons: u16,
    electrons_map: Vec<Vec<u8>>,
    aphelion_shell: u8,
    azimuthal_qn : u8,
    last_shell : u8
}


impl ElectronShell {
    pub fn from_electrons(num_of_electrons: usize) -> ElectronShell {
        const MAX_ELECTRONS: usize = 5000;

        // validate max num of electrons to prevent numeric variable calculation overflow
        if num_of_electrons > MAX_ELECTRONS {
            panic!(
                "Can only calculate with maximum of {} electrons",
                MAX_ELECTRONS
            );
        }

        let mut electrons_map: Vec<Vec<u8>> = Vec::new();
        let mut azimuthal_qn : u8 = 0;
        let mut aphelion_shell : u8 = 1;

        // iteration controllers
        let mut max_level: usize = 1; // current level
        let mut s: usize = 1; // current shell
        let mut es: usize = 0; // total electrons sum

        // shell loop
        loop {
            // increase the shell layer
            while electrons_map.len() < s {
                electrons_map.push(Vec::<u8>::new());
            }

            //  get the electron shell vec
            let electron_shell_vec = electrons_map.get_mut(s - 1).unwrap();

            // calculate the maximum amount of electrons in the current shell
            let mut max_electrons_in_s = (s * 2 - 1) * 2;

            // prevent exceeding the maximum amount of electrons
            if max_electrons_in_s + es > num_of_electrons {
                max_electrons_in_s = num_of_electrons - es;
            }

            // sum up electrons and add it on the vector
            es += max_electrons_in_s;
            electron_shell_vec.push(max_electrons_in_s as u8);

            // set the block
            if s as u8 > aphelion_shell {
                aphelion_shell = s as u8;
            }

            // set the az qn value
            azimuthal_qn = max_level as u8 - 1;

            // interrupt
            if es >= num_of_electrons {
                break;
            }

            // reached the 's' shell
            if s == 1 {

                max_level += 1;

                // the shell will always start on ceil(current level / 2)
                //if cl = 2 then next shell will be 1 (because ceil of 1)
                //if cl = 3 then next shell will be 1 (because ceil of 1.5)
                //if cl = 4 then next shell will be 2 (because ceil of 2)
                s = f32::ceil(max_level as f32 / 2.0) as usize;
            }
            // decrease shell level until reach the 's' shell
            else {
                s -= 1;
            }
        }

        ElectronShell {
            electrons: num_of_electrons as u16,
            electrons_map,
            aphelion_shell, 
            azimuthal_qn,
            last_shell : s as u8
        }
    }

    pub fn electron_subshell(&self) -> Vec<ElectronSubshell> {
        let mut vec : Vec<ElectronSubshell> = Vec::new();

        // go until the farthest  shell
        let mut max_shell : u8 = 1;
        for level_i in 0..=self.azimuthal_qn {

            for shell_i in 0..max_shell{
                let shell_i = shell_i as usize;
                let electron_i = level_i as usize - shell_i;

                // interrupt if there is no value in shell/electron coordinate
                match self.electrons_map.get(shell_i) {
                    Some(electrons_vec) => {

                        // interrupt if there is no electrons
                        match electrons_vec.get(electron_i) {
                            Some(e) => vec.push(ElectronSubshell { level: level_i + 1, shell: shell_i as u8, electrons: *e }),
                            None => break
                        }
                    },
                    None => break
                }
            }
            max_shell += 1;
        }

        // check if this is a d'block electron shell
        if self.last_shell == 3 {
            let s_block_index = vec.len() - 1;
            let d_block_index = vec.len() - 2;
            let d_block_e = vec.get(d_block_index).unwrap();

            // exception for 3d-series block
            if d_block_e.level == 3 {
                // if electrons in d'block shell are 4 or 9
                if d_block_e.electrons == 4 || d_block_e.electrons == 9 {
                    vec[s_block_index].electrons -= 1;
                    vec[d_block_index].electrons += 1;
                }
            }
            // exceptions for 4d-series block
            else if d_block_e.level == 4 {
                // if electrons in d'block shell are 4 or 9
                if d_block_e.electrons >= 3 && d_block_e.electrons <= 4 || d_block_e.electrons >= 6 && d_block_e.electrons <= 7 || d_block_e.electrons == 9{
                    vec[s_block_index].electrons -= 1;
                    vec[d_block_index].electrons += 1;
                }
                // when it is 8
                else if d_block_e.electrons == 8 {
                    vec.pop(); // remove the s block
                    vec[d_block_index].electrons += 2;
                }
            }
            // exceptions for 5d-series block
            else if d_block_e.level == 5 {
                if d_block_e.electrons >= 8 && d_block_e.electrons <= 9 {
                    vec[s_block_index].electrons -= 1;
                    vec[d_block_index].electrons += 1;
                }
            }
            // exceptions for 6d-series block
            else if d_block_e.level == 6 {
                // if electrons in d'block shell are 1 the last valence d block will be removed and replaced by an valence p block
                if d_block_e.electrons == 1 {
                    vec.swap(d_block_index, s_block_index); // put the s block electrons in the d block
                    vec.pop().unwrap(); // remove the d block
                    vec.push(ElectronSubshell { level: 7, shell: 1, electrons: 1 }) // include in the last block a 7p1
                }
            }
            
        }

        vec
    }

    pub fn electron_subshell_to_string(&self) -> String {
        self.electron_subshell().iter().map(|e| e.to_string()).collect()
    }

}

#[derive(Debug)]
pub struct Atom {
    protons: u16,
    neutrons: u16,
    electrons: u16,
    weight: f64,
    electron_shell: ElectronShell,
}

/// Store the Subshell labels used to describe an Atom electron configuration
pub static SUBSHELL_LABELS: &'static [char] =
    &['s', 'p', 'd', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm'];

fn subshell_label_from_az_value(az: usize) -> String {
    if az < SUBSHELL_LABELS.len() {
        return format!("{}", SUBSHELL_LABELS[az]);
    } else {
        return format!("n{}", (az - SUBSHELL_LABELS.len() + 1));
    }
}

impl Atom {
    pub fn from_p_e_n(protons: u16, electrons: u16, neutrons: u16) -> Atom {
        Atom {
            protons,
            neutrons,
            weight: (protons + neutrons) as f64,
            electrons,
            electron_shell: ElectronShell::from_electrons(electrons as usize),
        }
    }

    pub fn from_pe_n(protons_and_electrons: u16, neutrons: u16) -> Atom {
        Atom::from_p_e_n(protons_and_electrons, protons_and_electrons, neutrons)
    }

    pub fn from_pen(protons_electrons_and_neutrons: u16) -> Atom {
        Atom::from_pe_n(
            protons_electrons_and_neutrons,
            protons_electrons_and_neutrons,
        )
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
    pub fn is_isotope_of(&self, other: &Atom) -> bool {
        self.protons == other.protons
    }
}
