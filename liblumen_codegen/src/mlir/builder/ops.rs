mod builder;
pub(super) mod builders;

use std::fmt;

use libeir_intern::Ident;
use libeir_ir as ir;
use libeir_ir::FunctionIdent;

pub use self::builder::OpBuilder;

use crate::Result;

use super::block::Block;
use super::ffi::{self, Type};
use super::value::Value;
use super::ScopedFunctionBuilder;

/// Represents the different types of operations that can be present in an EIR block
#[derive(Debug, Clone)]
pub enum OpKind {
    Return(Option<Value>),
    Throw(Throw),
    Unreachable,
    Branch(Branch),
    Call(Call),
    If(If),
    IsType { value: Value, expected: Type },
    Match(Match),
    BinaryPush(BinaryPush),
    MapPut(MapPuts),
    BinOp(BinaryOperator),
    LogicOp(LogicalOperator),
    Constant(ir::Const),
    FunctionRef(Callee),
    Tuple(Vec<Value>),
    Cons(Value, Value),
    Map(Vec<(Value, Value)>),
    TraceCapture(Branch),
    TraceConstruct(Value),
    Intrinsic(Intrinsic),
}

#[derive(Debug, Clone, Copy)]
pub struct BinaryOperator {
    pub kind: ir::BinOp,
    pub lhs: Value,
    pub rhs: Value,
}

#[derive(Debug, Clone, Copy)]
pub struct LogicalOperator {
    pub kind: ir::LogicOp,
    pub lhs: Value,
    pub rhs: Option<Value>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Callee {
    Static(FunctionIdent),
    LocalDynamic {
        module: Ident,
        function: Value,
        arity: usize,
    },
    GlobalDynamic {
        module: Value,
        function: Value,
        arity: usize,
    },
}
impl Callee {
    pub fn new<'f, 'o>(
        builder: &mut ScopedFunctionBuilder<'f, 'o>,
        callee_value: ir::Value,
    ) -> Result<Self> {
        use libeir_ir::ValueKind;

        let op = builder.get_primop(callee_value);
        debug_assert_eq!(ir::PrimOpKind::CaptureFunction, *builder.primop_kind(op));
        let reads = builder.primop_reads(op);
        let num_reads = reads.len();
        // Figure out if this is a statically known function
        assert_eq!(3, num_reads, "expected 3 arguments to capture function op");

        // Resolve arity first, since we should always know arity
        let arity = builder.constant_int(builder.value_const(reads[2]));

        let m = reads[0];
        let f = reads[1];
        let mk = builder.value_kind(m);
        let callee = if let ValueKind::Const(mc) = mk {
            let module = builder.constant_atom(mc);
            let fk = builder.value_kind(f);
            if let ValueKind::Const(fc) = fk {
                let function = builder.constant_atom(fc);
                Self::Static(FunctionIdent {
                    module: Ident::with_empty_span(module),
                    name: Ident::with_empty_span(function),
                    arity: arity as usize,
                })
            } else {
                let function = builder.build_value(f)?;
                // Locally dynamic
                Self::LocalDynamic {
                    module: Ident::with_empty_span(module),
                    function,
                    arity: arity as usize,
                }
            }
        } else {
            let module = builder.build_value(m)?;
            let function = builder.build_value(f)?;
            // Globally dynamic
            Self::GlobalDynamic {
                module,
                function,
                arity: arity as usize,
            }
        };
        Ok(callee)
    }
}
impl fmt::Display for Callee {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Static(ref ident) => write!(f, "{}", ident),
            Self::LocalDynamic {
                module,
                function,
                arity,
            } => write!(f, "{}:{:?}/{}", module, function, arity),
            Self::GlobalDynamic {
                module,
                function,
                arity,
            } => write!(f, "{:?}:{:?}/{}", module, function, arity),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Call {
    pub callee: Callee,
    pub args: Vec<Value>,
    pub is_tail: bool,
    pub ok: CallSuccess,
    pub err: CallError,
}

#[derive(Debug, Clone)]
pub enum CallSuccess {
    Return,
    Branch(Branch),
}

#[derive(Debug, Clone)]
pub enum CallError {
    Throw,
    Branch(Branch),
}

#[derive(Debug, Clone)]
pub struct If {
    pub cond: Value,
    pub yes: Branch,
    pub no: Branch,
    pub otherwise: Option<Branch>,
}

#[derive(Debug, Clone)]
pub struct Branch {
    pub block: Block,
    pub args: Vec<Value>,
}

#[derive(Debug, Clone)]
pub struct Match {
    pub selector: Value,
    pub branches: Vec<Pattern>,
    pub reads: Vec<ir::Value>,
}

#[derive(Debug, Clone)]
pub struct Pattern {
    pub kind: ir::MatchKind,
    pub block: Block,
    pub args: Vec<ir::Value>,
}

#[derive(Debug, Clone)]
pub struct Intrinsic {
    pub name: libeir_intern::Symbol,
    pub args: Vec<ir::Value>,
}

#[derive(Debug, Clone)]
pub struct BinaryPush {
    pub ok: Block,
    pub err: Block,
    pub head: Value,
    pub tail: Value,
    pub size: Option<Value>,
    pub spec: ir::BinaryEntrySpecifier,
}

#[derive(Debug, Clone)]
pub struct MapPuts {
    pub ok: Block,
    pub err: Block,
    pub map: Value,
    pub puts: Vec<MapPut>,
}

#[derive(Debug, Clone)]
pub struct MapPut {
    pub action: ffi::MapActionType,
    pub key: Value,
    pub value: Value,
}

#[derive(Debug, Clone)]
pub struct Throw {
    pub kind: Value,
    pub class: Value,
    pub reason: Value,
}
