extern crate blah;
extern crate blah as beeble;
// line comment
use abc;
use abc as _;
use ::abc;
/// outer line doc comment
use crate::abc;
use crate::abc::AbcDef;
use crate::abc::ABCDEF;
use crate::abc::_ABCDEF;
use crate::abc::_ABC_DEF;
//! inner line doc comment
use super::abc;
/* block comment */
use self::abc::def;
/* block comment 
over multiple
lines*/
use abc as def;
use ::abc as def;
/* block comment 
/* nested over multiple */
lines*/
use crate::abc as def;
/** outer block doc comment */
use super::abc as def;
/** outer block doc comment 
over multiple
lines*/
use self::abc as def;
/** outer block doc comment 
/* nested over multiple */
lines*/
pub use abc;
/*! inner block doc comment */
pub use abc::def;
/*! inner block doc comment 
over multiple
lines*/
pub use crate::abc::def;
/*! inner block doc comment 
 TODO: blah
/* nested over multiple */
lines*/
pub use super::abc::def;
pub use self::abc::def;
use abc::*;
use crate::abc::*;
use super::abc::*;
use self::abc::*;
use self::abc::*;

use abc::{
	def::*,
	ghi
};
use crate::abc::{
	def::ghi::*,
	jkl as mno,
	def::PbandJ as PbandJ,
	def::{
		jkl as mno,
	}
}


pub mod abc {
	use abc;
	struct Abc;
	enum Abc;
	mod def;
	mod def {
		abc;
	};
};

let u: [A; 3] = abc;

#![InnerAttr(some_value = "blah")]
#![InnerAttr(some_value)]
#![InnerAttr(some_value(abc))]
#![InnerAttr(some_value[abc])]
#![InnerAttr(some_value{abc})]
#![InnerAttr(some_value, other_value, value_3)]

#[OuterAttr(some_value = "blah")]
#[OuterAttr(some_value)]
#[OuterAttr(some_value(abc))]
#[OuterAttr(some_value[abc])]
#[OuterAttr(some_value{abc})]
#[OuterAttr(some_value, other_value, value_3)]
struct Test;
struct Test<A>;
struct Test<A, B>;
struct Test<'a>;

struct Test(Type);
struct Test(Type) where Type: A + 'static;
struct Test(Type) where Type: A + 'static, B: C;
struct Test<A: B, C: D>(Type);
struct Test<A: B, C: D>(Type) where Type: A + 'static, B: C;
struct Test(pub Type);
struct Test(#[options] pub Type);
struct Test { value: Type };
struct Test { value: Type, };
struct Test {
	value: Type, 
	value2: Type2, 
};
pub struct Test;
pub struct Test(Type);
pub struct Test { value: Type }
pub struct Test { 0: Type, 1: Type<'a>, 2: (Type, Type) }
pub struct Test { value: Type, };
pub struct Test {
	value: Type, 
	value2: Type2, 
	0: Type2, 
	1: Type2, 
	2: Type2,
};
pub(crate) struct Test;
pub(crate) struct Test(Type);
pub(crate) struct Test { value: Type };
pub(crate) struct Test { value: Type, };
pub(crate) struct Test {
	value: Type, 
	value2: Type2, 
};
pub(in super) struct Test;
pub(in super) struct Test(Type);
pub(in super) struct Test { value: Type };
pub(in super) struct Test { value: Type, };
pub(in super) struct Test {
	value: Type, 
	value2: Type2, 
};
pub(in path::modu) struct Test;
pub(in path::modu) struct Test(Type);
pub(in path::modu) struct Test { value: Type };
pub(in path::modu) struct Test { value: Type, };
pub(in path::modu) struct Test {
	value: Type, 
	value2: Type2, 
};

enum Test;
enum Test<A>;
enum Test<A, B>;
enum Test<'a>;

enum Test{Type};
enum Test(Type) where Type: A + 'static;
enum Test(Type) where Type: A + 'static, B: C;
enum Test<A: B, C: D>{Type};
enum Test<A: B, C: D>(Type) where Type: A + 'static, B: C;
enum Test{pub Type};
enum Test{#[options] pub Type};
enum Test { Type };
enum Test { Type, };
enum Test {
	Type, 
	Type2, 
};
pub enum Test;
pub enum Test{Type};
pub enum Test { Type }
pub enum Test { Type1, Type2(Type), Type3(Type) }
pub enum Test { Type, }
pub enum Test {
	#[Attribute(abc)]
	TestType {
		value: Type, 
		#[Attribute(abc)]
		value2: Type2, 
		0: Type2, 
		1: Type2, 
		2: Type2,
	}
};
pub(crate) enum Test;
pub(crate) enum Test{Type};
pub(crate) enum Test { Type };
pub(crate) enum Test { Type, };
pub(crate) enum Test {
	Type, 
	#[Attribute(abc)]
	Type2, 
};
pub(crate) enum Test {
	#[Attribute(abc)]
	Type = 0, 
	#[Attribute(abc)]
	Type2 = 1, 
};

union Test;
union Test<A>;
union Test<A, B>;
union Test<'a>;
union Test { value: Type };
union Test { value: Type, };
union Test {
	value: Type, 
	value2: Type2, 
};
pub union Test;
pub union Test { value: Type }
pub union Test { 0: Type, 1: Type<'a>, 2: (Type, Type) }
pub union Test { value: Type, };
pub union Test {
	value: Type, 
	value2: Type2, 
	0: Type2, 
	1: Type2, 
	2: Type2,
};
pub(crate) union Test;
pub(crate) union Test { value: Type };
pub(crate) union Test { value: Type, };
pub(crate) union Test {
	value: Type, 
	value2: Type2, 
};
pub(in super) union Test;
pub(in super) union Test(Type);
pub(in super) union Test { value: Type };
pub(in super) union Test { value: Type, };
pub(in super) union Test {
	value: Type, 
	value2: Type2, 
};
pub(in path::modu) union Test;
pub(in path::modu) union Test { value: Type };
pub(in path::modu) union Test { value: Type, };
pub(in path::modu) union Test {
	value: Type, 
	value2: Type2, 
};


fn main() {}
fn main() -> () {}
fn main<T>() {}
fn main<T>() -> () {}
fn main(arg: Type) {}
fn main(arg: Type) -> () {}
fn main<T>(arg: Type) {}
fn main<T>(arg: Type) -> () {}
fn main(arg: Type, arg2: Type2) {}
fn main(arg: Type, arg2: Type2) -> () {}
fn main<T>(arg: Type, arg2: Type2) {}
fn main<T>(arg: Type, arg2: Type2) -> () {}
pub fn main() {}
pub fn main() -> () {}
pub fn main<T>() {}
pub fn main<T>() -> () {}
pub fn main(arg: Type) {}
pub fn main(arg: Type) -> () {}
pub fn main<T>(arg: Type) {}
pub fn main<T>(arg: Type) -> () {}
pub fn main(arg: Type, arg2: Type2) {}
pub fn main(arg: Type, arg2: Type2) -> () {}
pub fn main<T>(arg: Type, arg2: Type2) {}
pub fn main<T>(arg: Type, arg2: Type2) -> () {}
const fn main() {}
const fn main() -> TypeRet {}
const fn main<T>() {}
const fn main<T>() -> TypeRet {}
const fn main(arg: Type) {}
const fn main(arg: Type) -> TypeRet {}
const fn main<T>(arg: Type) {}
const fn main<T>(arg: Type) -> TypeRet {}
const fn main(arg: Type, arg2: Type2) {}
const fn main(arg: Type, arg2: Type2) -> TypeRet {}
const fn main<T>(arg: Type, arg2: Type2) {}
const fn main<T>(arg: Type, arg2: Type2) -> TypeRet {}
extern fn main() -> Result<Ret, Erro> {}
extern fn main<T>() -> Result<Ret, Erro> {}
extern fn main(arg: Type) -> Result<Ret, Erro> {}
extern fn main<T>(arg: Type) -> Result<Ret, Erro> {}
extern fn main(arg: Type, arg2: Type2) -> Result<Ret, Erro> {}
extern fn main<T>(arg: Type, arg2: Type2) -> Result<Ret, Erro> {}
extern "C" fn main<T, U>() {}
extern "C" fn main<T, U>() -> TypeRet {}
extern "C" fn main<T, U>(arg: Type) {}
extern "C" fn main<T, U>(arg: Type) -> TypeRet {}
extern "C" fn main<T, U>(arg: Type, arg2: Type2) {}
extern "C" fn main<T, U>(arg: Type, arg2: Type2) -> TypeRet {}
crate fn main<T, U: Bound, V: Bound + 'a>() {}
crate fn main<T, U: Bound, V: Bound + 'a>() -> TypeRet {}
crate fn main<T, U: Bound, V: Bound + 'a>(arg: Type) {}
crate fn main<T, U: Bound, V: Bound + 'a>(arg: Type) -> TypeRet {}
crate fn main<T, U: Bound, V: Bound + 'a>(arg: Type, arg2: Type2) {}
crate fn main<T, U: Bound, V: Bound + 'a>(arg: Type, arg2: Type2) -> TypeRet {}
fn main<T, U>() where T: Bound {}
fn main<T, U>() -> TypeRet where T: Bound {}
fn main<T, U>(arg: Type) where T: Bound {}
fn main<T, U>(arg: Type) -> TypeRet where T: Bound {}
fn main<T, U>(arg: Type, arg2: Type2) where T: Bound {}
fn main<T, U>(arg: Type, arg2: Type2) -> TypeRet where T: Bound {}
fn main<T, U>() where T: Bound + 'a {}
fn main<T, U>() -> TypeRet where T: Bound + 'a {}
fn main<T, U>(arg: Type) where T: Bound + 'a {}
fn main<T, U>(arg: Type) -> TypeRet where T: Bound + 'a {}
fn main<T, U>(arg: Type, arg2: Type2) where T: Bound + 'a {}
fn main<T, U>(arg: Type, arg2: Type2) -> TypeRet where T: Bound + 'a {}


pub type Abc = Def;
pub type Abc = abc::Def;
pub type Abc = abc::Def<A, B>;
pub type Abc<T> where B: C = Result<A, B>;
pub type Abc<T> where B: C = abc::Def<A, B>;
type Abc<T> where B: C = abc::Def<A, B>;

pub const ABCDEF: Type = blah;

const fn abc() {}

trait Bingo {}
unsafe trait Bingo: Debug {}
pub trait Bingo: Debug {}

trait Bingo {
	type Blah;
	type Blah: Debug + 'some_life;
	fn abc(&self);
	fn abc(&self) {

	}
	const VALUE: Type = blah;
}

fn abc() -> impl (dyn Adf + Send) {}


fn main() {
	do box

	let a = 1234?;
	let a = 1234;
	let a = 0x1234;
	let a = 0x1234ABCDEF;
	let a = 0x1234ABCDEFG;
	let a: u32 = 0o1284;
	let a: u32 = 0o1274;
	let a: u32 = 0b1010;
	let a = 0b10102u32;

	let h: bool = true;
	let h: (bool, _) = (false, true);
	let x = <Type>::a;
	let x = <Type as Default>::a;
	let s = "string";
	let s = r#"string\n\n\n"#;
	let s = "string\n\n\n";
	let s = b"string";
	let s = br###"string"###;
	let r#type = br###"string"###;
	let s = format!("{:?} {:<#0.2}", s, 234f32);
	let s = format!("{} {s}", s, s = s);
	let c = 'a';
	let c = '\n';

	'label: loop {
		;;;
	}

}


impl  Def {
	#[inline(always)]
	pub fn make(&self, blah: &mut Type) -> () {
	}

	#[inline]
	fn make(&self, blah: Type) -> ! {
		
	}

	fn make(&self, blah: Type) -> [u32; 2] {
		
	}


	fn make(&self, blah: Type) -> const * Type {
		
	}

	fn make(&self, blah: Type) -> &&Type {
		
	}

	fn make(&self, blah: Type) -> _ {
		
	}


	fn make(&self, blah: Type) -> impl Iterator<Item=&f32> {
		
	}
}
