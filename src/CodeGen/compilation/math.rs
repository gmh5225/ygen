use crate::CodeGen::instr::{MachineMnemonic, MachineOperand, MachineInstr};
use crate::IR::ir::*;
use crate::prelude::{Var, Block, Type};
use super::{CompilationHelper, VarLocation};

macro_rules! MathVarVar {
    ($func:ident, $node:ident, $mnemonic:expr) => {
        impl CompilationHelper {
            #[allow(missing_docs)]
            pub(crate) fn $func(&mut self, node: &$node<Var, Var, Var>, mc_sink: &mut Vec<MachineInstr>, block: &Block) {
                let src1 = *self.vars.get(&node.inner1.name).expect("expected valid variable");
                let src2 = *self.vars.get(&node.inner2.name).expect("expected valid variable");
        
                let boxed: Box<dyn Ir> = Box::new(node.clone());
        
                if !block.isVarUsedAfterNode(&boxed, &node.inner1) { // op1
                    self.free(&node.inner1);
                }
                if !block.isVarUsedAfterNode(&boxed, &node.inner2) { // op2
                    self.free(&node.inner2);
                }
                if !block.isVarUsedAfterNode(&boxed, &node.inner3) { // out
                    return; 
                }
        
                let out = self.alloc(&node.inner3);
        
                let mut instr = MachineInstr::new($mnemonic);
        
                match src1 {
                    VarLocation::Reg(reg) => instr.add_operand(MachineOperand::Reg(reg)),
                }
        
                match src2 {
                    VarLocation::Reg(reg) => instr.add_operand(MachineOperand::Reg(reg)),
                }
        
                match out {
                    VarLocation::Reg(reg) => instr.set_out(MachineOperand::Reg(reg)),
                }
                
                instr.meta = node.inner3.ty;
        
                mc_sink.push(instr);
            }
        }
    };
}


MathVarVar!(compile_add_var_var, Add, MachineMnemonic::Add);
MathVarVar!(compile_and_var_var, And, MachineMnemonic::And);
MathVarVar!(compile_div_var_var, Div, MachineMnemonic::Div);
MathVarVar!(compile_mul_var_var, Mul, MachineMnemonic::Mul);
MathVarVar!(compile_or_var_var, Or, MachineMnemonic::Or);
MathVarVar!(compile_sub_var_var, Sub, MachineMnemonic::Sub);
MathVarVar!(compile_xor_var_var, Xor, MachineMnemonic::Xor);


macro_rules! MathVarType {
    ($func:ident, $node:ident, $mnemonic:expr) => {
        impl CompilationHelper {
            #[allow(missing_docs)]
            pub fn $func(&mut self, node: &$node<Var, Type, Var>, mc_sink: &mut Vec<MachineInstr>, block: &Block) {
                let src1 = *self.vars.get(&node.inner1.name).expect("expected valid variable");
        
                let boxed: Box<dyn Ir> = Box::new(node.clone());
        
                if !block.isVarUsedAfterNode(&boxed, &node.inner1) { // op1
                    self.free(&node.inner1);
                }
                if !block.isVarUsedAfterNode(&boxed, &node.inner3) { // out
                    return; 
                }
        
                let out = self.alloc(&node.inner3);
        
                let mut instr = MachineInstr::new($mnemonic);
        
                match src1 {
                    VarLocation::Reg(reg) => instr.add_operand(MachineOperand::Reg(reg)),
                }
        
                instr.add_operand(MachineOperand::Imm(node.inner2.val() as i64));

                match out {
                    VarLocation::Reg(reg) => instr.set_out(MachineOperand::Reg(reg)),
                }

                
                instr.meta = node.inner3.ty;
        
                mc_sink.push(instr);
            }
        }
    };
}


MathVarType!(compile_add_var_type, Add, MachineMnemonic::Add);
MathVarType!(compile_and_var_type, And, MachineMnemonic::And);
MathVarType!(compile_div_var_type, Div, MachineMnemonic::Div);
MathVarType!(compile_mul_var_type, Mul, MachineMnemonic::Mul);
MathVarType!(compile_or_var_type, Or, MachineMnemonic::Or);
MathVarType!(compile_sub_var_type, Sub, MachineMnemonic::Sub);
MathVarType!(compile_xor_var_type, Xor, MachineMnemonic::Xor);


macro_rules! MathTypeType {
    ($func:ident, $node:ident, $mnemonic:expr) => {
        impl CompilationHelper {
            #[allow(missing_docs)]
            pub(crate) fn $func(&mut self, node: &$node<Type, Type, Var>, mc_sink: &mut Vec<MachineInstr>, block: &Block) {
                let boxed: Box<dyn Ir> = Box::new(node.clone());

                if !block.isVarUsedAfterNode(&boxed, &node.inner3) { // out
                    return; 
                }
        
                let out = self.alloc(&node.inner3);
        
                let mut instr = MachineInstr::new($mnemonic);
        
                instr.add_operand(MachineOperand::Imm(node.inner1.val() as i64));
        
                instr.add_operand(MachineOperand::Imm(node.inner2.val() as i64));

                match out {
                    VarLocation::Reg(reg) => instr.set_out(MachineOperand::Reg(reg)),
                }
        
                instr.meta = node.inner3.ty;

                mc_sink.push(instr);
            }
        }
    };
}


MathTypeType!(compile_add_type_type, Add, MachineMnemonic::Add);
MathTypeType!(compile_and_type_type, And, MachineMnemonic::And);
MathTypeType!(compile_div_type_type, Div, MachineMnemonic::Div);
MathTypeType!(compile_mul_type_type, Mul, MachineMnemonic::Mul);
MathTypeType!(compile_or_type_type, Or, MachineMnemonic::Or);
MathTypeType!(compile_sub_type_type, Sub, MachineMnemonic::Sub);
MathTypeType!(compile_xor_type_type, Xor, MachineMnemonic::Xor);
