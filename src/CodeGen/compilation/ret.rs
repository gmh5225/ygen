use crate::{prelude::Return, CodeGen::{MachineInstr, MachineMnemonic, MachineOperand}, IR::{Block, Type, Var}};

use super::CompilationHelper;

impl CompilationHelper {
    #[allow(missing_docs)]
    pub fn compile_ret_ty(&mut self, node: &Return<Type>, mc_sink: &mut Vec<MachineInstr>, _: &Block) {
        let mut instr = MachineInstr::new(MachineMnemonic::Move);

        instr.set_out(MachineOperand::Reg(self.call.return_reg(self.arch, node.inner1.into())) );
        instr.add_operand(MachineOperand::Imm(node.inner1.val() as i64));

        mc_sink.push( instr );
        
        mc_sink.push( MachineInstr::new(MachineMnemonic::Return) );
    }

    #[allow(missing_docs)]
    pub fn compile_ret_var(&mut self, node: &Return<Var>, mc_sink: &mut Vec<MachineInstr>, _: &Block) {
        let src = *self.vars.get(&node.inner1.name).expect("expected valid variable");
        
        let mut instr = MachineInstr::new(MachineMnemonic::Move);

        instr.set_out(MachineOperand::Reg(self.call.return_reg(self.arch, node.inner1.ty)) );
        
        match src {
            super::VarLocation::Reg(reg) => instr.add_operand(MachineOperand::Reg(reg)),
        }

        mc_sink.push(instr);

        mc_sink.push( MachineInstr::new(MachineMnemonic::Return) );
    }
}