#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused)]

use std::collections::HashMap;
use std::mem;

use cranelift::prelude::*;
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::{DataDescription, Linkage, Module};

use crate::interpreter::ast::{BinaryOp, Expr, Program, Statement, UnaryOp, Value};
use crate::interpreter::value;



#[repr(u8)]
#[derive(Clone, Copy)]
pub enum BinOpCode {
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    BitAnd,
    BitOr,
    BitXor,
    Equal,
    NotEqual,
    Greater,
    Less,
    GreaterEqual,
    LessEqual,
    And,
    Or,
}

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum UnOpCode {
    Minus,
    Not,
    BitNot,
}

fn bin_op_code(op: &BinaryOp) -> BinOpCode {
    match op {
        BinaryOp::Plus => BinOpCode::Plus,
        BinaryOp::Minus => BinOpCode::Minus,
        BinaryOp::Star => BinOpCode::Star,
        BinaryOp::Slash => BinOpCode::Slash,
        BinaryOp::Percent => BinOpCode::Percent,
        BinaryOp::BitAnd => BinOpCode::BitAnd,
        BinaryOp::BitOr => BinOpCode::BitOr,
        BinaryOp::BitXor => BinOpCode::BitXor,
        BinaryOp::Equal => BinOpCode::Equal,
        BinaryOp::NotEqual => BinOpCode::NotEqual,
        BinaryOp::Greater => BinOpCode::Greater,
        BinaryOp::Less => BinOpCode::Less,
        BinaryOp::GreaterEqual => BinOpCode::GreaterEqual,
        BinaryOp::LessEqual => BinOpCode::LessEqual,
        BinaryOp::And => BinOpCode::And,
        BinaryOp::Or => BinOpCode::Or,
    }
}

fn un_op_code(op: &UnaryOp) -> UnOpCode {
    match op {
        UnaryOp::Minus => UnOpCode::Minus,
        UnaryOp::Not => UnOpCode::Not,
        UnaryOp::BitNot => UnOpCode::BitNot,
    }
}


struct RuntimeFuncIds {
    binary_op: cranelift_module::FuncId,
    unary_op: cranelift_module::FuncId,
    truthy: cranelift_module::FuncId,
    show: cranelift_module::FuncId,
    read: cranelift_module::FuncId,
}

pub struct Jit {
    builder_context: FunctionBuilderContext,
    ctx: cranelift_codegen::Context,
    data_description: DataDescription,
    module: JITModule,
}

impl Jit {
    pub fn new() -> Self {
        let mut flag_builder = settings::builder();
        flag_builder.set("use_colocated_libcalls", "false").unwrap();
        flag_builder.set("is_pic", "false").unwrap();
        let isa_builder = cranelift_native::builder().expect("ISA nativa indisponibila");
        let isa = isa_builder
            .finish(settings::Flags::new(flag_builder))
            .expect("nu pot construi ISA-ul");

        let mut jit_builder =
            JITBuilder::with_isa(isa, cranelift_module::default_libcall_names());

        jit_builder.symbol("rt_binary_op", value::rt_binary_op as *const u8);
        jit_builder.symbol("rt_unary_op", value::rt_unary_op as *const u8);
        jit_builder.symbol("rt_truthy", value::rt_truthy as *const u8);
        jit_builder.symbol("rt_show", value::rt_show as *const u8);
        jit_builder.symbol("rt_read", value::rt_read as *const u8);

        let module = JITModule::new(jit_builder);

        Self {
            builder_context: FunctionBuilderContext::new(),
            ctx: module.make_context(),
            data_description: DataDescription::new(),
            module,
        }
    }

    
    pub fn compile_and_run(&mut self, program: &Program) -> Result<i64, String> {
        let runtime = self.declare_runtime_functions()?;

        
        self.ctx.func.signature.returns.push(AbiParam::new(types::I64));

        let func_id = self
            .module
            .declare_function("airy_main", Linkage::Export, &self.ctx.func.signature)
            .map_err(|e| e.to_string())?;

        {
            let mut builder = FunctionBuilder::new(&mut self.ctx.func, &mut self.builder_context);

            let entry_block = builder.create_block();
            builder.append_block_params_for_function_params(entry_block);
            builder.switch_to_block(entry_block);
            builder.seal_block(entry_block);

            let mut translator = FunctionTranslator {
                builder,
                module: &mut self.module,
                runtime: &runtime,
                vars: HashMap::new(),
                var_index: 0,
                loop_stack: Vec::new(),
                terminated: false,
            };

            translator.translate_block(&program.statements);

            
            if !translator.terminated {
                let zero = translator.builder.ins().iconst(types::I64, 0);
                translator.builder.ins().return_(&[zero]);
            }

            translator.builder.finalize();
        }

        self.module
            .define_function(func_id, &mut self.ctx)
            .map_err(|e| e.to_string())?;

        self.module.clear_context(&mut self.ctx);
        self.module.finalize_definitions().map_err(|e| e.to_string())?;

        let code_ptr = self.module.get_finalized_function(func_id);
        let main_fn = unsafe { mem::transmute::<_, extern "C" fn() -> i64>(code_ptr) };

        Ok(main_fn())
    }

    fn declare_runtime_functions(&mut self) -> Result<RuntimeFuncIds, String> {
        let mut sig_binary = self.module.make_signature();
        sig_binary.params.push(AbiParam::new(types::I64)); 
        sig_binary.params.push(AbiParam::new(types::I64)); 
        sig_binary.params.push(AbiParam::new(types::I64)); 
        sig_binary.returns.push(AbiParam::new(types::I64));
        let binary_op = self
            .module
            .declare_function("rt_binary_op", Linkage::Import, &sig_binary)
            .map_err(|e| e.to_string())?;

        let mut sig_unary = self.module.make_signature();
        sig_unary.params.push(AbiParam::new(types::I64)); 
        sig_unary.params.push(AbiParam::new(types::I64)); 
        sig_unary.returns.push(AbiParam::new(types::I64));
        let unary_op = self
            .module
            .declare_function("rt_unary_op", Linkage::Import, &sig_unary)
            .map_err(|e| e.to_string())?;

        let mut sig_truthy = self.module.make_signature();
        sig_truthy.params.push(AbiParam::new(types::I64));
        sig_truthy.returns.push(AbiParam::new(types::I64));
        let truthy = self
            .module
            .declare_function("rt_truthy", Linkage::Import, &sig_truthy)
            .map_err(|e| e.to_string())?;

        let mut sig_show = self.module.make_signature();
        sig_show.params.push(AbiParam::new(types::I64));
        let show = self
            .module
            .declare_function("rt_show", Linkage::Import, &sig_show)
            .map_err(|e| e.to_string())?;

        let mut sig_read = self.module.make_signature();
        sig_read.returns.push(AbiParam::new(types::I64));
        let read = self
            .module
            .declare_function("rt_read", Linkage::Import, &sig_read)
            .map_err(|e| e.to_string())?;

        Ok(RuntimeFuncIds {
            binary_op,
            unary_op,
            truthy,
            show,
            read,
        })
    }
}


struct LoopExit {
    exit_block: Block_,
}


type Block_ = cranelift::prelude::Block;

struct FunctionTranslator<'a> {
    builder: FunctionBuilder<'a>,
    module: &'a mut JITModule,
    runtime: &'a RuntimeFuncIds,
    vars: HashMap<String, Variable>,
    var_index: usize,
    loop_stack: Vec<LoopExit>,
    
    
    
    
    terminated: bool,
}

impl<'a> FunctionTranslator<'a> {
    fn switch_to(&mut self, block: Block_) {
        self.builder.switch_to_block(block);
        self.terminated = false;
    }

    fn translate_block(&mut self, stmts: &[Statement]) {
        for stmt in stmts {
            if self.terminated {
                break;
            }
            self.translate_statement(stmt);
        }
    }

    fn translate_statement(&mut self, stmt: &Statement) {
        {
            match stmt {
                Statement::Set { name, value } => {
                    let val = match value {
                        Some(expr) => self.translate_expr(expr),
                        None => self.builder.ins().iconst(types::I64, value::encode_bool(false)),
                    };
                    let var = self.get_or_declare_var(name);
                    self.builder.def_var(var, val);
                }

                Statement::Assign { name, value } => {
                    let val = self.translate_expr(value);
                    let var = self.get_or_declare_var(name);
                    self.builder.def_var(var, val);
                }

                Statement::Read { name } => {
                    let local_callee = self
                        .module
                        .declare_func_in_func(self.runtime.read, self.builder.func);
                    let call = self.builder.ins().call(local_callee, &[]);
                    let result = self.builder.inst_results(call)[0];
                    let var = self.get_or_declare_var(name);
                    self.builder.def_var(var, result);
                }

                Statement::Show { value } => {
                    let val = self.translate_expr(value);
                    let local_callee = self
                        .module
                        .declare_func_in_func(self.runtime.show, self.builder.func);
                    self.builder.ins().call(local_callee, &[val]);
                }

                Statement::If {
                    condition,
                    body,
                    elseif_branches,
                    else_body,
                } => {
                    self.translate_if(condition, body, elseif_branches, else_body);
                }

                Statement::Loop {
                    variable,
                    start,
                    end,
                    body,
                } => {
                    self.translate_loop(variable, start, end, body);
                }

                Statement::InfLoop { condition, body } => {
                    self.translate_infloop(condition, body);
                }

                Statement::Break => {
                    if let Some(loop_exit) = self.loop_stack.last() {
                        let exit = loop_exit.exit_block;
                        self.builder.ins().jump(exit, &[]);
                        self.terminated = true;
                    } else {
                        panic!("'break' folosit in afara unei bucle");
                    }
                }

                Statement::Return => {
                    let zero = self.builder.ins().iconst(types::I64, 0);
                    self.builder.ins().return_(&[zero]);
                    self.terminated = true;
                }
            }
        }
    }

    fn get_or_declare_var(&mut self, name: &str) -> Variable {
        if let Some(v) = self.vars.get(name) {
            return *v;
        }

        let var = self.builder.declare_var(types::I64);

        self.vars.insert(name.to_string(), var);
        var
    }

    
    fn translate_condition(&mut self, expr: &Expr) -> Value_ {
        let tagged = self.translate_expr(expr);
        let local_callee = self
            .module
            .declare_func_in_func(self.runtime.truthy, self.builder.func);
        let call = self.builder.ins().call(local_callee, &[tagged]);
        let truthy_i64 = self.builder.inst_results(call)[0];
        self.builder
            .ins()
            .icmp_imm(IntCC::NotEqual, truthy_i64, 0)
    }

    fn translate_if(
        &mut self,
        condition: &Expr,
        body: &[Statement],
        elseif_branches: &[(Expr, Vec<Statement>)],
        else_body: &Option<Vec<Statement>>,
    ) {
        let merge_block = self.builder.create_block();

        self.translate_if_chain(condition, body, elseif_branches, else_body, merge_block);

        self.switch_to(merge_block);
        self.builder.seal_block(merge_block);
    }

    fn translate_if_chain(
        &mut self,
        condition: &Expr,
        body: &[Statement],
        elseif_branches: &[(Expr, Vec<Statement>)],
        else_body: &Option<Vec<Statement>>,
        merge_block: Block_,
    ) {
        let cond_val = self.translate_condition(condition);

        let then_block = self.builder.create_block();
        let next_block = self.builder.create_block();

        self.builder
            .ins()
            .brif(cond_val, then_block, &[], next_block, &[]);

        self.switch_to(then_block);
        self.builder.seal_block(then_block);
        self.translate_block(body);
        if !self.terminated {
            self.builder.ins().jump(merge_block, &[]);
        }

        self.switch_to(next_block);
        self.builder.seal_block(next_block);

        if let Some((next_cond, next_body)) = elseif_branches.first() {
            self.translate_if_chain(
                next_cond,
                next_body,
                &elseif_branches[1..],
                else_body,
                merge_block,
            );
        } else if let Some(eb) = else_body {
            self.translate_block(eb);
            if !self.terminated {
                self.builder.ins().jump(merge_block, &[]);
            }
        } else {
            self.builder.ins().jump(merge_block, &[]);
        }
    }

    fn translate_loop(&mut self, variable: &str, start: &Expr, end: &Expr, body: &[Statement]) {
        let start_val = self.translate_expr(start);
        let end_val = self.translate_expr(end);

        let var = self.get_or_declare_var(variable);
        self.builder.def_var(var, start_val);

        let header_block = self.builder.create_block();
        let body_block = self.builder.create_block();
        let exit_block = self.builder.create_block();

        self.builder.ins().jump(header_block, &[]);

        
        self.switch_to(header_block);
        let i_val = self.builder.use_var(var);
        let i_int = self.builder.ins().sshr_imm(i_val, value::TAG_BITS as i64);
        let end_int = self.builder.ins().sshr_imm(end_val, value::TAG_BITS as i64);
        let cmp = self
            .builder
            .ins()
            .icmp(IntCC::SignedLessThan, i_int, end_int);
        self.builder.ins().brif(cmp, body_block, &[], exit_block, &[]);

        self.switch_to(body_block);
        self.builder.seal_block(body_block);

        self.loop_stack.push(LoopExit { exit_block });
        self.translate_block(body);
        self.loop_stack.pop();

        if !self.terminated {
            
            let cur = self.builder.use_var(var);
            let step = self.builder.ins().iconst(types::I64, 1 << value::TAG_BITS);
            let next = self.builder.ins().iadd(cur, step);
            self.builder.def_var(var, next);
            self.builder.ins().jump(header_block, &[]);
        }

        self.builder.seal_block(header_block);
        self.switch_to(exit_block);
        self.builder.seal_block(exit_block);
    }

    fn translate_infloop(&mut self, condition: &Expr, body: &[Statement]) {
        let header_block = self.builder.create_block();
        let body_block = self.builder.create_block();
        let exit_block = self.builder.create_block();

        self.builder.ins().jump(header_block, &[]);

        self.switch_to(header_block);
        let cond_val = self.translate_condition(condition);
        self.builder.ins().brif(cond_val, body_block, &[], exit_block, &[]);

        self.switch_to(body_block);
        self.builder.seal_block(body_block);

        self.loop_stack.push(LoopExit { exit_block });
        self.translate_block(body);
        self.loop_stack.pop();

        if !self.terminated {
            self.builder.ins().jump(header_block, &[]);
        }

        self.builder.seal_block(header_block);
        self.switch_to(exit_block);
        self.builder.seal_block(exit_block);
    }

    fn translate_expr(&mut self, expr: &Expr) -> Value_ {
        match expr {
            Expr::Literal(Value::Integer(n)) => {
                self.builder.ins().iconst(types::I64, value::encode_int(*n))
            }
            Expr::Literal(Value::Boolean(b)) => {
                self.builder.ins().iconst(types::I64, value::encode_bool(*b))
            }
            Expr::Literal(Value::Float(f)) => {
                
                
                let tagged = value::encode_float(*f);
                self.builder.ins().iconst(types::I64, tagged)
            }
            Expr::Literal(Value::String(s)) => {
                let tagged = value::encode_string(s.clone());
                self.builder.ins().iconst(types::I64, tagged)
            }

            Expr::Identifier(name) => {
                let var = self.get_or_declare_var(name);
                self.builder.use_var(var)
            }

            Expr::Binary { left, op, right } => {
                let l = self.translate_expr(left);
                let r = self.translate_expr(right);
                let code = bin_op_code(op);
                let code_val = self.builder.ins().iconst(types::I64, code as i64);

                let local_callee = self
                    .module
                    .declare_func_in_func(self.runtime.binary_op, self.builder.func);
                let call = self.builder.ins().call(local_callee, &[code_val, l, r]);
                self.builder.inst_results(call)[0]
            }

            Expr::Unary { op, expr } => {
                let v = self.translate_expr(expr);
                let code = un_op_code(op);
                let code_val = self.builder.ins().iconst(types::I64, code as i64);

                let local_callee = self
                    .module
                    .declare_func_in_func(self.runtime.unary_op, self.builder.func);
                let call = self.builder.ins().call(local_callee, &[code_val, v]);
                self.builder.inst_results(call)[0]
            }

            Expr::Perform(inner) => {
                
                self.translate_expr(inner)
            }
        }
    }
}


type Value_ = cranelift::prelude::Value;
