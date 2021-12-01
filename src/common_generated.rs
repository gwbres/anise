// automatically generated by the FlatBuffers compiler, do not modify



use std::mem;
use std::cmp::Ordering;

extern crate flatbuffers;
use self::flatbuffers::{EndianScalar, Follow};

#[allow(unused_imports, dead_code)]
pub mod anise {

  use std::mem;
  use std::cmp::Ordering;

  extern crate flatbuffers;
  use self::flatbuffers::{EndianScalar, Follow};
#[allow(unused_imports, dead_code)]
pub mod common {

  use std::mem;
  use std::cmp::Ordering;

  extern crate flatbuffers;
  use self::flatbuffers::{EndianScalar, Follow};

#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MIN_INTERPOLATION_KIND: u8 = 0;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MAX_INTERPOLATION_KIND: u8 = 4;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
#[allow(non_camel_case_types)]
pub const ENUM_VALUES_INTERPOLATION_KIND: [InterpolationKind; 5] = [
  InterpolationKind::ChebyshevSeries,
  InterpolationKind::HermiteSeries,
  InterpolationKind::LagrangeSeries,
  InterpolationKind::Polynomial,
  InterpolationKind::Trigonometric,
];

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct InterpolationKind(pub u8);
#[allow(non_upper_case_globals)]
impl InterpolationKind {
  pub const ChebyshevSeries: Self = Self(0);
  pub const HermiteSeries: Self = Self(1);
  pub const LagrangeSeries: Self = Self(2);
  pub const Polynomial: Self = Self(3);
  pub const Trigonometric: Self = Self(4);

  pub const ENUM_MIN: u8 = 0;
  pub const ENUM_MAX: u8 = 4;
  pub const ENUM_VALUES: &'static [Self] = &[
    Self::ChebyshevSeries,
    Self::HermiteSeries,
    Self::LagrangeSeries,
    Self::Polynomial,
    Self::Trigonometric,
  ];
  /// Returns the variant's name or "" if unknown.
  pub fn variant_name(self) -> Option<&'static str> {
    match self {
      Self::ChebyshevSeries => Some("ChebyshevSeries"),
      Self::HermiteSeries => Some("HermiteSeries"),
      Self::LagrangeSeries => Some("LagrangeSeries"),
      Self::Polynomial => Some("Polynomial"),
      Self::Trigonometric => Some("Trigonometric"),
      _ => None,
    }
  }
}
impl std::fmt::Debug for InterpolationKind {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    if let Some(name) = self.variant_name() {
      f.write_str(name)
    } else {
      f.write_fmt(format_args!("<UNKNOWN {:?}>", self.0))
    }
  }
}
impl<'a> flatbuffers::Follow<'a> for InterpolationKind {
  type Inner = Self;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    let b = unsafe {
      flatbuffers::read_scalar_at::<u8>(buf, loc)
    };
    Self(b)
  }
}

impl flatbuffers::Push for InterpolationKind {
    type Output = InterpolationKind;
    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        unsafe { flatbuffers::emplace_scalar::<u8>(dst, self.0); }
    }
}

impl flatbuffers::EndianScalar for InterpolationKind {
  #[inline]
  fn to_little_endian(self) -> Self {
    let b = u8::to_le(self.0);
    Self(b)
  }
  #[inline]
  #[allow(clippy::wrong_self_convention)]
  fn from_little_endian(self) -> Self {
    let b = u8::from_le(self.0);
    Self(b)
  }
}

impl<'a> flatbuffers::Verifiable for InterpolationKind {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    u8::run_verifier(v, pos)
  }
}

impl flatbuffers::SimpleToVerifyInSlice for InterpolationKind {}
// struct Vector3, aligned to 8
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq)]
pub struct Vector3(pub [u8; 24]);
impl Default for Vector3 { 
  fn default() -> Self { 
    Self([0; 24])
  }
}
impl std::fmt::Debug for Vector3 {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    f.debug_struct("Vector3")
      .field("x", &self.x())
      .field("y", &self.y())
      .field("z", &self.z())
      .finish()
  }
}

impl flatbuffers::SimpleToVerifyInSlice for Vector3 {}
impl flatbuffers::SafeSliceAccess for Vector3 {}
impl<'a> flatbuffers::Follow<'a> for Vector3 {
  type Inner = &'a Vector3;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    <&'a Vector3>::follow(buf, loc)
  }
}
impl<'a> flatbuffers::Follow<'a> for &'a Vector3 {
  type Inner = &'a Vector3;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    flatbuffers::follow_cast_ref::<Vector3>(buf, loc)
  }
}
impl<'b> flatbuffers::Push for Vector3 {
    type Output = Vector3;
    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        let src = unsafe {
            ::std::slice::from_raw_parts(self as *const Vector3 as *const u8, Self::size())
        };
        dst.copy_from_slice(src);
    }
}
impl<'b> flatbuffers::Push for &'b Vector3 {
    type Output = Vector3;

    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        let src = unsafe {
            ::std::slice::from_raw_parts(*self as *const Vector3 as *const u8, Self::size())
        };
        dst.copy_from_slice(src);
    }
}

impl<'a> flatbuffers::Verifiable for Vector3 {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.in_buffer::<Self>(pos)
  }
}
impl<'a> Vector3 {
  #[allow(clippy::too_many_arguments)]
  pub fn new(
    x: f64,
    y: f64,
    z: f64,
  ) -> Self {
    let mut s = Self([0; 24]);
    s.set_x(x);
    s.set_y(y);
    s.set_z(z);
    s
  }

  pub fn x(&self) -> f64 {
    let mut mem = core::mem::MaybeUninit::<f64>::uninit();
    unsafe {
      core::ptr::copy_nonoverlapping(
        self.0[0..].as_ptr(),
        mem.as_mut_ptr() as *mut u8,
        core::mem::size_of::<f64>(),
      );
      mem.assume_init()
    }.from_little_endian()
  }

  pub fn set_x(&mut self, x: f64) {
    let x_le = x.to_little_endian();
    unsafe {
      core::ptr::copy_nonoverlapping(
        &x_le as *const f64 as *const u8,
        self.0[0..].as_mut_ptr(),
        core::mem::size_of::<f64>(),
      );
    }
  }

  pub fn y(&self) -> f64 {
    let mut mem = core::mem::MaybeUninit::<f64>::uninit();
    unsafe {
      core::ptr::copy_nonoverlapping(
        self.0[8..].as_ptr(),
        mem.as_mut_ptr() as *mut u8,
        core::mem::size_of::<f64>(),
      );
      mem.assume_init()
    }.from_little_endian()
  }

  pub fn set_y(&mut self, x: f64) {
    let x_le = x.to_little_endian();
    unsafe {
      core::ptr::copy_nonoverlapping(
        &x_le as *const f64 as *const u8,
        self.0[8..].as_mut_ptr(),
        core::mem::size_of::<f64>(),
      );
    }
  }

  pub fn z(&self) -> f64 {
    let mut mem = core::mem::MaybeUninit::<f64>::uninit();
    unsafe {
      core::ptr::copy_nonoverlapping(
        self.0[16..].as_ptr(),
        mem.as_mut_ptr() as *mut u8,
        core::mem::size_of::<f64>(),
      );
      mem.assume_init()
    }.from_little_endian()
  }

  pub fn set_z(&mut self, x: f64) {
    let x_le = x.to_little_endian();
    unsafe {
      core::ptr::copy_nonoverlapping(
        &x_le as *const f64 as *const u8,
        self.0[16..].as_mut_ptr(),
        core::mem::size_of::<f64>(),
      );
    }
  }

}

// struct Quaternion, aligned to 8
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq)]
pub struct Quaternion(pub [u8; 32]);
impl Default for Quaternion { 
  fn default() -> Self { 
    Self([0; 32])
  }
}
impl std::fmt::Debug for Quaternion {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    f.debug_struct("Quaternion")
      .field("w", &self.w())
      .field("x", &self.x())
      .field("y", &self.y())
      .field("z", &self.z())
      .finish()
  }
}

impl flatbuffers::SimpleToVerifyInSlice for Quaternion {}
impl flatbuffers::SafeSliceAccess for Quaternion {}
impl<'a> flatbuffers::Follow<'a> for Quaternion {
  type Inner = &'a Quaternion;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    <&'a Quaternion>::follow(buf, loc)
  }
}
impl<'a> flatbuffers::Follow<'a> for &'a Quaternion {
  type Inner = &'a Quaternion;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    flatbuffers::follow_cast_ref::<Quaternion>(buf, loc)
  }
}
impl<'b> flatbuffers::Push for Quaternion {
    type Output = Quaternion;
    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        let src = unsafe {
            ::std::slice::from_raw_parts(self as *const Quaternion as *const u8, Self::size())
        };
        dst.copy_from_slice(src);
    }
}
impl<'b> flatbuffers::Push for &'b Quaternion {
    type Output = Quaternion;

    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        let src = unsafe {
            ::std::slice::from_raw_parts(*self as *const Quaternion as *const u8, Self::size())
        };
        dst.copy_from_slice(src);
    }
}

impl<'a> flatbuffers::Verifiable for Quaternion {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.in_buffer::<Self>(pos)
  }
}
impl<'a> Quaternion {
  #[allow(clippy::too_many_arguments)]
  pub fn new(
    w: f64,
    x: f64,
    y: f64,
    z: f64,
  ) -> Self {
    let mut s = Self([0; 32]);
    s.set_w(w);
    s.set_x(x);
    s.set_y(y);
    s.set_z(z);
    s
  }

  pub fn w(&self) -> f64 {
    let mut mem = core::mem::MaybeUninit::<f64>::uninit();
    unsafe {
      core::ptr::copy_nonoverlapping(
        self.0[0..].as_ptr(),
        mem.as_mut_ptr() as *mut u8,
        core::mem::size_of::<f64>(),
      );
      mem.assume_init()
    }.from_little_endian()
  }

  pub fn set_w(&mut self, x: f64) {
    let x_le = x.to_little_endian();
    unsafe {
      core::ptr::copy_nonoverlapping(
        &x_le as *const f64 as *const u8,
        self.0[0..].as_mut_ptr(),
        core::mem::size_of::<f64>(),
      );
    }
  }

  pub fn x(&self) -> f64 {
    let mut mem = core::mem::MaybeUninit::<f64>::uninit();
    unsafe {
      core::ptr::copy_nonoverlapping(
        self.0[8..].as_ptr(),
        mem.as_mut_ptr() as *mut u8,
        core::mem::size_of::<f64>(),
      );
      mem.assume_init()
    }.from_little_endian()
  }

  pub fn set_x(&mut self, x: f64) {
    let x_le = x.to_little_endian();
    unsafe {
      core::ptr::copy_nonoverlapping(
        &x_le as *const f64 as *const u8,
        self.0[8..].as_mut_ptr(),
        core::mem::size_of::<f64>(),
      );
    }
  }

  pub fn y(&self) -> f64 {
    let mut mem = core::mem::MaybeUninit::<f64>::uninit();
    unsafe {
      core::ptr::copy_nonoverlapping(
        self.0[16..].as_ptr(),
        mem.as_mut_ptr() as *mut u8,
        core::mem::size_of::<f64>(),
      );
      mem.assume_init()
    }.from_little_endian()
  }

  pub fn set_y(&mut self, x: f64) {
    let x_le = x.to_little_endian();
    unsafe {
      core::ptr::copy_nonoverlapping(
        &x_le as *const f64 as *const u8,
        self.0[16..].as_mut_ptr(),
        core::mem::size_of::<f64>(),
      );
    }
  }

  pub fn z(&self) -> f64 {
    let mut mem = core::mem::MaybeUninit::<f64>::uninit();
    unsafe {
      core::ptr::copy_nonoverlapping(
        self.0[24..].as_ptr(),
        mem.as_mut_ptr() as *mut u8,
        core::mem::size_of::<f64>(),
      );
      mem.assume_init()
    }.from_little_endian()
  }

  pub fn set_z(&mut self, x: f64) {
    let x_le = x.to_little_endian();
    unsafe {
      core::ptr::copy_nonoverlapping(
        &x_le as *const f64 as *const u8,
        self.0[24..].as_mut_ptr(),
        core::mem::size_of::<f64>(),
      );
    }
  }

}

pub enum ConstantOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct Constant<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Constant<'a> {
    type Inner = Constant<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self { _tab: flatbuffers::Table { buf, loc } }
    }
}

impl<'a> Constant<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        Constant { _tab: table }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args ConstantArgs<'args>) -> flatbuffers::WIPOffset<Constant<'bldr>> {
      let mut builder = ConstantBuilder::new(_fbb);
      builder.add_value(args.value);
      if let Some(x) = args.comment { builder.add_comment(x); }
      if let Some(x) = args.unit { builder.add_unit(x); }
      builder.finish()
    }

    pub const VT_VALUE: flatbuffers::VOffsetT = 4;
    pub const VT_UNIT: flatbuffers::VOffsetT = 6;
    pub const VT_COMMENT: flatbuffers::VOffsetT = 8;

  #[inline]
  pub fn value(&self) -> f64 {
    self._tab.get::<f64>(Constant::VT_VALUE, Some(0.0)).unwrap()
  }
  #[inline]
  pub fn unit(&self) -> Option<&'a str> {
    self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(Constant::VT_UNIT, None)
  }
  #[inline]
  pub fn comment(&self) -> Option<&'a str> {
    self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(Constant::VT_COMMENT, None)
  }
}

impl flatbuffers::Verifiable for Constant<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<f64>(&"value", Self::VT_VALUE, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>(&"unit", Self::VT_UNIT, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>(&"comment", Self::VT_COMMENT, false)?
     .finish();
    Ok(())
  }
}
pub struct ConstantArgs<'a> {
    pub value: f64,
    pub unit: Option<flatbuffers::WIPOffset<&'a str>>,
    pub comment: Option<flatbuffers::WIPOffset<&'a str>>,
}
impl<'a> Default for ConstantArgs<'a> {
    #[inline]
    fn default() -> Self {
        ConstantArgs {
            value: 0.0,
            unit: None,
            comment: None,
        }
    }
}
pub struct ConstantBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> ConstantBuilder<'a, 'b> {
  #[inline]
  pub fn add_value(&mut self, value: f64) {
    self.fbb_.push_slot::<f64>(Constant::VT_VALUE, value, 0.0);
  }
  #[inline]
  pub fn add_unit(&mut self, unit: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Constant::VT_UNIT, unit);
  }
  #[inline]
  pub fn add_comment(&mut self, comment: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Constant::VT_COMMENT, comment);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> ConstantBuilder<'a, 'b> {
    let start = _fbb.start_table();
    ConstantBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Constant<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl std::fmt::Debug for Constant<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut ds = f.debug_struct("Constant");
      ds.field("value", &self.value());
      ds.field("unit", &self.unit());
      ds.field("comment", &self.comment());
      ds.finish()
  }
}
pub enum ConstantMapOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct ConstantMap<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for ConstantMap<'a> {
    type Inner = ConstantMap<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self { _tab: flatbuffers::Table { buf, loc } }
    }
}

impl<'a> ConstantMap<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        ConstantMap { _tab: table }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args ConstantMapArgs<'args>) -> flatbuffers::WIPOffset<ConstantMap<'bldr>> {
      let mut builder = ConstantMapBuilder::new(_fbb);
      if let Some(x) = args.values { builder.add_values(x); }
      if let Some(x) = args.keys { builder.add_keys(x); }
      builder.finish()
    }

    pub const VT_KEYS: flatbuffers::VOffsetT = 4;
    pub const VT_VALUES: flatbuffers::VOffsetT = 6;

  #[inline]
  pub fn keys(&self) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<&'a str>>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<&'a str>>>>(ConstantMap::VT_KEYS, None)
  }
  #[inline]
  pub fn values(&self) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<Constant<'a>>>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<Constant>>>>(ConstantMap::VT_VALUES, None)
  }
}

impl flatbuffers::Verifiable for ConstantMap<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, flatbuffers::ForwardsUOffset<&'_ str>>>>(&"keys", Self::VT_KEYS, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, flatbuffers::ForwardsUOffset<Constant>>>>(&"values", Self::VT_VALUES, false)?
     .finish();
    Ok(())
  }
}
pub struct ConstantMapArgs<'a> {
    pub keys: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<&'a str>>>>,
    pub values: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<Constant<'a>>>>>,
}
impl<'a> Default for ConstantMapArgs<'a> {
    #[inline]
    fn default() -> Self {
        ConstantMapArgs {
            keys: None,
            values: None,
        }
    }
}
pub struct ConstantMapBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> ConstantMapBuilder<'a, 'b> {
  #[inline]
  pub fn add_keys(&mut self, keys: flatbuffers::WIPOffset<flatbuffers::Vector<'b , flatbuffers::ForwardsUOffset<&'b  str>>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(ConstantMap::VT_KEYS, keys);
  }
  #[inline]
  pub fn add_values(&mut self, values: flatbuffers::WIPOffset<flatbuffers::Vector<'b , flatbuffers::ForwardsUOffset<Constant<'b >>>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(ConstantMap::VT_VALUES, values);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> ConstantMapBuilder<'a, 'b> {
    let start = _fbb.start_table();
    ConstantMapBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<ConstantMap<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl std::fmt::Debug for ConstantMap<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut ds = f.debug_struct("ConstantMap");
      ds.field("keys", &self.keys());
      ds.field("values", &self.values());
      ds.finish()
  }
}
}  // pub mod Common
}  // pub mod Anise

