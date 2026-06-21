#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused)]

use std::io::Write;

pub const TAG_BITS: i64 = 3;
pub const TAG_MASK: i64 = 0b111;

pub const TAG_INT: i64 = 0b000;
pub const TAG_BOOL: i64 = 0b001;
pub const TAG_FLOAT: i64 = 0b010;
pub const TAG_STRING: i64 = 0b011;

#[inline]
pub fn tag_of(v: i64) -> i64 {
    v & TAG_MASK
}

#[inline]
pub fn encode_int(n: i64) -> i64 {
    (n << TAG_BITS) | TAG_INT
}

#[inline]
pub fn decode_int(v: i64) -> i64 {
    v >> TAG_BITS
}

#[inline]
pub fn encode_bool(b: bool) -> i64 {
    ((b as i64) << TAG_BITS) | TAG_BOOL
}

#[inline]
pub fn decode_bool(v: i64) -> bool {
    (v >> TAG_BITS) != 0
}

#[inline]
pub fn encode_float(f: f64) -> i64 {
    let boxed: &'static mut f64 = Box::leak(Box::new(f));
    let ptr = boxed as *mut f64 as i64;
    debug_assert_eq!(ptr & TAG_MASK, 0, "pointer needs to be alligned at 8 octets");
    ptr | TAG_FLOAT
}

#[inline]
pub fn decode_float(v: i64) -> f64 {
    let ptr = (v & !TAG_MASK) as *const f64;
    unsafe { *ptr }
}

#[inline]
pub fn encode_string(s: String) -> i64 {
    let boxed: &'static mut String = Box::leak(Box::new(s));
    let ptr = boxed as *mut String as i64;
    debug_assert_eq!(ptr & TAG_MASK, 0, "pointer needs to be alligned at 8 octets");
    ptr | TAG_STRING
}

#[inline]
pub fn decode_string<'a>(v: i64) -> &'a String {
    let ptr = (v & !TAG_MASK) as *const String;
    unsafe { &*ptr }
}


pub enum RtValue {
    Int(i64),
    Float(f64),
    Bool(bool),
    Str(String),
}

pub fn decode(v: i64) -> RtValue {
    match tag_of(v) {
        TAG_INT => RtValue::Int(decode_int(v)),
        TAG_BOOL => RtValue::Bool(decode_bool(v)),
        TAG_FLOAT => RtValue::Float(decode_float(v)),
        TAG_STRING => RtValue::Str(decode_string(v).clone()),
        _ => unreachable!("uknown tag"),
    }
}

pub fn encode(v: RtValue) -> i64 {
    match v {
        RtValue::Int(n) => encode_int(n),
        RtValue::Float(f) => encode_float(f),
        RtValue::Bool(b) => encode_bool(b),
        RtValue::Str(s) => encode_string(s),
    }
}

fn as_f64(v: i64) -> f64 {
    match tag_of(v) {
        TAG_INT => decode_int(v) as f64,
        TAG_FLOAT => decode_float(v),
        TAG_BOOL => decode_bool(v) as i64 as f64,
        _ => panic!("cannot convert string to number"),
    }
}

fn is_float_op(a: i64, b: i64) -> bool {
    tag_of(a) == TAG_FLOAT || tag_of(b) == TAG_FLOAT
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_binary_op(op_code: i64, a: i64, b: i64) -> i64 {
    use super::jit::BinOpCode::*;
    let op: super::jit::BinOpCode = unsafe { std::mem::transmute(op_code as u8) };

    match op {
        Plus => {
            if tag_of(a) == TAG_STRING || tag_of(b) == TAG_STRING {
                let s = format!("{}{}", display_value(a), display_value(b));
                encode_string(s)
            } else if is_float_op(a, b) {
                encode_float(as_f64(a) + as_f64(b))
            } else {
                encode_int(decode_int(a) + decode_int(b))
            }
        }
        Minus => {
            if is_float_op(a, b) {
                encode_float(as_f64(a) - as_f64(b))
            } else {
                encode_int(decode_int(a) - decode_int(b))
            }
        }
        Star => {
            if is_float_op(a, b) {
                encode_float(as_f64(a) * as_f64(b))
            } else {
                encode_int(decode_int(a) * decode_int(b))
            }
        }
        Slash => {
            if is_float_op(a, b) {
                encode_float(as_f64(a) / as_f64(b))
            } else {
                let denom = decode_int(b);
                if denom == 0 {
                    panic!("zero dividing error");
                }
                encode_int(decode_int(a) / denom)
            }
        }
        Percent => encode_int(decode_int(a) % decode_int(b)),

        BitAnd => encode_int(decode_int(a) & decode_int(b)),
        BitOr => encode_int(decode_int(a) | decode_int(b)),
        BitXor => encode_int(decode_int(a) ^ decode_int(b)),

        Equal => encode_bool(values_equal(a, b)),
        NotEqual => encode_bool(!values_equal(a, b)),
        Greater => encode_bool(as_f64(a) > as_f64(b)),
        Less => encode_bool(as_f64(a) < as_f64(b)),
        GreaterEqual => encode_bool(as_f64(a) >= as_f64(b)),
        LessEqual => encode_bool(as_f64(a) <= as_f64(b)),

        And => encode_bool(truthy(a) && truthy(b)),
        Or => encode_bool(truthy(a) || truthy(b)),
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_unary_op(op_code: i64, a: i64) -> i64 {
    use super::jit::UnOpCode::*;
    let op: super::jit::UnOpCode = unsafe { std::mem::transmute(op_code as u8) };

    match op {
        Minus => {
            if tag_of(a) == TAG_FLOAT {
                encode_float(-decode_float(a))
            } else {
                encode_int(-decode_int(a))
            }
        }
        Not => encode_bool(!truthy(a)),
        BitNot => encode_int(!decode_int(a)),
    }
}


#[unsafe(no_mangle)]
pub extern "C" fn rt_truthy(v: i64) -> i64 {
    truthy(v) as i64
}

pub fn truthy(v: i64) -> bool {
    match tag_of(v) {
        TAG_INT => decode_int(v) != 0,
        TAG_BOOL => decode_bool(v),
        TAG_FLOAT => decode_float(v) != 0.0,
        TAG_STRING => !decode_string(v).is_empty(),
        _ => unreachable!(),
    }
}

fn values_equal(a: i64, b: i64) -> bool {
    match (tag_of(a), tag_of(b)) {
        (TAG_STRING, TAG_STRING) => decode_string(a) == decode_string(b),
        (TAG_STRING, _) | (_, TAG_STRING) => false,
        _ => as_f64(a) == as_f64(b),
    }
}

fn display_value(v: i64) -> String {
    match decode(v) {
        RtValue::Int(n) => n.to_string(),
        RtValue::Float(f) => f.to_string(),
        RtValue::Bool(b) => b.to_string(),
        RtValue::Str(s) => s,
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_show(v: i64) {
    println!("{}", display_value(v));
}

#[unsafe(no_mangle)]
pub extern "C" fn rt_read() -> i64 {
    print!("> ");
    std::io::stdout().flush().ok();

    let mut line = String::new();
    std::io::stdin()
        .read_line(&mut line)
        .expect("error at reading from  standart input (stdin)");
    let line = line.trim().to_string();

    if let Ok(n) = line.parse::<i64>() {
        encode_int(n)
    } else if let Ok(f) = line.parse::<f64>() {
        encode_float(f)
    } else if line == "true" || line == "false" {
        encode_bool(line == "true")
    } else {
        encode_string(line)
    }
}