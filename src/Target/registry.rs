use std::{collections::HashMap, error::Error, fmt::Display};

use crate::{prelude::{Block, Function}, CodeGen::MachineInstr, Obj::Link};

use super::{Arch, CallConv, TargetBackendDescr, Triple};

/// The target registry: manages different targets
pub struct TargetRegistry {
    targets: HashMap<Arch, TargetBackendDescr>,
    triple: Triple,
}

impl TargetRegistry {
    /// Creates an new backend registry
    pub fn new(triple: Triple) -> Self {
        Self {
            targets: HashMap::new(),
            triple: triple,
        }
    }

    /// Adds an new target architecture
    pub fn add(&mut self, arch: Arch, descr: TargetBackendDescr) {
        self.targets.insert(arch, descr);
    }

    /// Sets the calling convention to use for the specified architecture
    /// If it isn't found the function does noting
    pub fn setCallingConventionForTarget(&mut self, arch: Arch, call: CallConv) {
        if let Some(target) = self.targets.get_mut(&arch) {
            target.call = call;
        }
    }

    /// returns the `TargetBackendDescr` for the arch (also it adjusts it's calling convention ...)
    pub fn getBasedOnArch(&mut self, arch: Arch) -> Result<&mut TargetBackendDescr, Box<dyn Error>> {
        if let Some(descr) = self.targets.get_mut(&arch) {
            Ok(descr)
        } else {
            Err(Box::from( 
                RegistryError::UnsuportedArch(arch) 
            ))
        }
    }

    /// emits machine instrs for target
    /// note: machine instrs are portable over all platforms
    pub fn buildMachineInstrsForTarget(&mut self, arch: Arch, block: &Block, funct: &Function) -> Result<Vec<MachineInstr>, Box<dyn Error>> {
        let triple = self.triple;
        let org = self.getBasedOnArch(arch)?;

        org.block = Some(block.clone());
        let instrs = org.build_instrs(&funct, &triple);

        org.reset();

        Ok(instrs)
    }

    /// Builds the ir of the given triple into text assembly code
    pub fn buildAsmForTarget(&mut self, arch: Arch, block: &Block, funct: &Function) -> Result<Vec<String>, Box<dyn Error>> {
       let triple = self.triple;

        let org = self.getBasedOnArch(arch)?;
        org.block = Some(block.clone());

        let instrs = org.build_instrs(&funct, &triple);
        let instrs = org.lower(instrs)?;

        let mut asm = vec![];

        for instr in instrs {
            asm.push(
                instr.to_string()
            )
        }
        
        org.reset();

        Ok(asm)
    }

    /// Builds the ir of the given triple into machine code
    pub fn buildMachineCodeForTarget(&mut self, arch: Arch, block: &Block, funct: &Function) -> Result<(Vec<u8>, Vec<Link>), Box<dyn Error>> {
        let triple = self.triple;

        let org = self.getBasedOnArch(arch)?;

        org.block = Some(block.clone());

        let instrs = org.build_instrs(&funct, &triple);
        let instrs = org.lower(instrs)?;

        let mut res = vec![];
        let mut links = vec![];

        for instr in &instrs {
            let (encoded, link) = &instr.encode()?;
            res.extend_from_slice(&encoded);

            if let Some(link) = link {
                let mut link = link.clone();

                if link.special {
                    link.from = block.name.to_owned();
                } else {
                    link.from = funct.name.to_string();
                }
                link.at = res.len();

                links.push(link);
            }
        }

        org.reset();

        Ok((res, links))
    }
}

/// Stores errors which can occure in the `getBasedOnTriple` function in the `TargetRegistry`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegistryError {
    /// An unsupported architecture
    UnsuportedArch(Arch),
}

impl Display for RegistryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            RegistryError::UnsuportedArch(arch) => format!("unsuported architecture: {:?}", arch),
        })
    }
}

impl Error for RegistryError {}