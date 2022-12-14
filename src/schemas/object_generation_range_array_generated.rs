// automatically generated by the FlatBuffers compiler, do not modify


// @generated
use core::mem;
use core::cmp::Ordering;

extern crate flatbuffers;
use self::flatbuffers::{EndianScalar, Follow};

#[allow(unused_imports, dead_code, non_snake_case)]
pub mod ogra {

  use core::mem;
  use core::cmp::Ordering;

  extern crate flatbuffers;
  use self::flatbuffers::{EndianScalar, Follow};

#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MIN_OBJECT_TYPE: i32 = -1;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MAX_OBJECT_TYPE: i32 = 8;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
#[allow(non_camel_case_types)]
pub const ENUM_VALUES_OBJECT_TYPE: [ObjectType; 10] = [
  ObjectType::None,
  ObjectType::RaidGem,
  ObjectType::EncountPokemon,
  ObjectType::DistantViewEffect,
  ObjectType::GroundEffect,
  ObjectType::HiddenItem,
  ObjectType::CrashRock,
  ObjectType::TrafficNpc,
  ObjectType::FieldNpc,
  ObjectType::FixEncout,
];

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct ObjectType(pub i32);
#[allow(non_upper_case_globals)]
impl ObjectType {
  pub const None: Self = Self(-1);
  pub const RaidGem: Self = Self(0);
  pub const EncountPokemon: Self = Self(1);
  pub const DistantViewEffect: Self = Self(2);
  pub const GroundEffect: Self = Self(3);
  pub const HiddenItem: Self = Self(4);
  pub const CrashRock: Self = Self(5);
  pub const TrafficNpc: Self = Self(6);
  pub const FieldNpc: Self = Self(7);
  pub const FixEncout: Self = Self(8);

  pub const ENUM_MIN: i32 = -1;
  pub const ENUM_MAX: i32 = 8;
  pub const ENUM_VALUES: &'static [Self] = &[
    Self::None,
    Self::RaidGem,
    Self::EncountPokemon,
    Self::DistantViewEffect,
    Self::GroundEffect,
    Self::HiddenItem,
    Self::CrashRock,
    Self::TrafficNpc,
    Self::FieldNpc,
    Self::FixEncout,
  ];
  /// Returns the variant's name or "" if unknown.
  pub fn variant_name(self) -> Option<&'static str> {
    match self {
      Self::None => Some("None"),
      Self::RaidGem => Some("RaidGem"),
      Self::EncountPokemon => Some("EncountPokemon"),
      Self::DistantViewEffect => Some("DistantViewEffect"),
      Self::GroundEffect => Some("GroundEffect"),
      Self::HiddenItem => Some("HiddenItem"),
      Self::CrashRock => Some("CrashRock"),
      Self::TrafficNpc => Some("TrafficNpc"),
      Self::FieldNpc => Some("FieldNpc"),
      Self::FixEncout => Some("FixEncout"),
      _ => None,
    }
  }
}
impl core::fmt::Debug for ObjectType {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    if let Some(name) = self.variant_name() {
      f.write_str(name)
    } else {
      f.write_fmt(format_args!("<UNKNOWN {:?}>", self.0))
    }
  }
}
impl<'a> flatbuffers::Follow<'a> for ObjectType {
  type Inner = Self;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    let b = flatbuffers::read_scalar_at::<i32>(buf, loc);
    Self(b)
  }
}

impl flatbuffers::Push for ObjectType {
    type Output = ObjectType;
    #[inline]
    unsafe fn push(&self, dst: &mut [u8], _written_len: usize) {
        flatbuffers::emplace_scalar::<i32>(dst, self.0);
    }
}

impl flatbuffers::EndianScalar for ObjectType {
  type Scalar = i32;
  #[inline]
  fn to_little_endian(self) -> i32 {
    self.0.to_le()
  }
  #[inline]
  #[allow(clippy::wrong_self_convention)]
  fn from_little_endian(v: i32) -> Self {
    let b = i32::from_le(v);
    Self(b)
  }
}

impl<'a> flatbuffers::Verifiable for ObjectType {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    i32::run_verifier(v, pos)
  }
}

impl flatbuffers::SimpleToVerifyInSlice for ObjectType {}
pub enum ObjectGenerationRangeOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct ObjectGenerationRange<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for ObjectGenerationRange<'a> {
  type Inner = ObjectGenerationRange<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> ObjectGenerationRange<'a> {
  pub const VT_TYPE_: flatbuffers::VOffsetT = 4;
  pub const VT_MINCREATEDISTANCE: flatbuffers::VOffsetT = 6;
  pub const VT_MAXCREATEDISTANCE: flatbuffers::VOffsetT = 8;
  pub const VT_MINDESTROYDISTANCE: flatbuffers::VOffsetT = 10;
  pub const VT_MAXDESTROYDISTANCE: flatbuffers::VOffsetT = 12;
  pub const VT_MAXGENERATIONNUM: flatbuffers::VOffsetT = 14;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    ObjectGenerationRange { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args ObjectGenerationRangeArgs
  ) -> flatbuffers::WIPOffset<ObjectGenerationRange<'bldr>> {
    let mut builder = ObjectGenerationRangeBuilder::new(_fbb);
    builder.add_maxGenerationNum(args.maxGenerationNum);
    builder.add_maxDestroyDistance(args.maxDestroyDistance);
    builder.add_minDestroyDistance(args.minDestroyDistance);
    builder.add_maxCreateDistance(args.maxCreateDistance);
    builder.add_minCreateDistance(args.minCreateDistance);
    builder.add_type_(args.type_);
    builder.finish()
  }


  #[inline]
  pub fn type_(&self) -> ObjectType {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<ObjectType>(ObjectGenerationRange::VT_TYPE_, Some(ObjectType::RaidGem)).unwrap()}
  }
  #[inline]
  pub fn minCreateDistance(&self) -> f32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<f32>(ObjectGenerationRange::VT_MINCREATEDISTANCE, Some(0.0)).unwrap()}
  }
  #[inline]
  pub fn maxCreateDistance(&self) -> f32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<f32>(ObjectGenerationRange::VT_MAXCREATEDISTANCE, Some(0.0)).unwrap()}
  }
  #[inline]
  pub fn minDestroyDistance(&self) -> f32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<f32>(ObjectGenerationRange::VT_MINDESTROYDISTANCE, Some(0.0)).unwrap()}
  }
  #[inline]
  pub fn maxDestroyDistance(&self) -> f32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<f32>(ObjectGenerationRange::VT_MAXDESTROYDISTANCE, Some(0.0)).unwrap()}
  }
  #[inline]
  pub fn maxGenerationNum(&self) -> i32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<i32>(ObjectGenerationRange::VT_MAXGENERATIONNUM, Some(0)).unwrap()}
  }
}

impl flatbuffers::Verifiable for ObjectGenerationRange<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<ObjectType>("type_", Self::VT_TYPE_, false)?
     .visit_field::<f32>("minCreateDistance", Self::VT_MINCREATEDISTANCE, false)?
     .visit_field::<f32>("maxCreateDistance", Self::VT_MAXCREATEDISTANCE, false)?
     .visit_field::<f32>("minDestroyDistance", Self::VT_MINDESTROYDISTANCE, false)?
     .visit_field::<f32>("maxDestroyDistance", Self::VT_MAXDESTROYDISTANCE, false)?
     .visit_field::<i32>("maxGenerationNum", Self::VT_MAXGENERATIONNUM, false)?
     .finish();
    Ok(())
  }
}
pub struct ObjectGenerationRangeArgs {
    pub type_: ObjectType,
    pub minCreateDistance: f32,
    pub maxCreateDistance: f32,
    pub minDestroyDistance: f32,
    pub maxDestroyDistance: f32,
    pub maxGenerationNum: i32,
}
impl<'a> Default for ObjectGenerationRangeArgs {
  #[inline]
  fn default() -> Self {
    ObjectGenerationRangeArgs {
      type_: ObjectType::RaidGem,
      minCreateDistance: 0.0,
      maxCreateDistance: 0.0,
      minDestroyDistance: 0.0,
      maxDestroyDistance: 0.0,
      maxGenerationNum: 0,
    }
  }
}

pub struct ObjectGenerationRangeBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> ObjectGenerationRangeBuilder<'a, 'b> {
  #[inline]
  pub fn add_type_(&mut self, type_: ObjectType) {
    self.fbb_.push_slot::<ObjectType>(ObjectGenerationRange::VT_TYPE_, type_, ObjectType::RaidGem);
  }
  #[inline]
  pub fn add_minCreateDistance(&mut self, minCreateDistance: f32) {
    self.fbb_.push_slot::<f32>(ObjectGenerationRange::VT_MINCREATEDISTANCE, minCreateDistance, 0.0);
  }
  #[inline]
  pub fn add_maxCreateDistance(&mut self, maxCreateDistance: f32) {
    self.fbb_.push_slot::<f32>(ObjectGenerationRange::VT_MAXCREATEDISTANCE, maxCreateDistance, 0.0);
  }
  #[inline]
  pub fn add_minDestroyDistance(&mut self, minDestroyDistance: f32) {
    self.fbb_.push_slot::<f32>(ObjectGenerationRange::VT_MINDESTROYDISTANCE, minDestroyDistance, 0.0);
  }
  #[inline]
  pub fn add_maxDestroyDistance(&mut self, maxDestroyDistance: f32) {
    self.fbb_.push_slot::<f32>(ObjectGenerationRange::VT_MAXDESTROYDISTANCE, maxDestroyDistance, 0.0);
  }
  #[inline]
  pub fn add_maxGenerationNum(&mut self, maxGenerationNum: i32) {
    self.fbb_.push_slot::<i32>(ObjectGenerationRange::VT_MAXGENERATIONNUM, maxGenerationNum, 0);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> ObjectGenerationRangeBuilder<'a, 'b> {
    let start = _fbb.start_table();
    ObjectGenerationRangeBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<ObjectGenerationRange<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for ObjectGenerationRange<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("ObjectGenerationRange");
      ds.field("type_", &self.type_());
      ds.field("minCreateDistance", &self.minCreateDistance());
      ds.field("maxCreateDistance", &self.maxCreateDistance());
      ds.field("minDestroyDistance", &self.minDestroyDistance());
      ds.field("maxDestroyDistance", &self.maxDestroyDistance());
      ds.field("maxGenerationNum", &self.maxGenerationNum());
      ds.finish()
  }
}
pub enum ObjectGenerationRangeArrayOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct ObjectGenerationRangeArray<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for ObjectGenerationRangeArray<'a> {
  type Inner = ObjectGenerationRangeArray<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> ObjectGenerationRangeArray<'a> {
  pub const VT_VALUES: flatbuffers::VOffsetT = 4;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    ObjectGenerationRangeArray { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args ObjectGenerationRangeArrayArgs<'args>
  ) -> flatbuffers::WIPOffset<ObjectGenerationRangeArray<'bldr>> {
    let mut builder = ObjectGenerationRangeArrayBuilder::new(_fbb);
    if let Some(x) = args.values { builder.add_values(x); }
    builder.finish()
  }


  #[inline]
  pub fn values(&self) -> flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<ObjectGenerationRange<'a>>> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<ObjectGenerationRange>>>>(ObjectGenerationRangeArray::VT_VALUES, Some(Default::default())).unwrap()}
  }
}

impl flatbuffers::Verifiable for ObjectGenerationRangeArray<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, flatbuffers::ForwardsUOffset<ObjectGenerationRange>>>>("values", Self::VT_VALUES, false)?
     .finish();
    Ok(())
  }
}
pub struct ObjectGenerationRangeArrayArgs<'a> {
    pub values: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<ObjectGenerationRange<'a>>>>>,
}
impl<'a> Default for ObjectGenerationRangeArrayArgs<'a> {
  #[inline]
  fn default() -> Self {
    ObjectGenerationRangeArrayArgs {
      values: None,
    }
  }
}

pub struct ObjectGenerationRangeArrayBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> ObjectGenerationRangeArrayBuilder<'a, 'b> {
  #[inline]
  pub fn add_values(&mut self, values: flatbuffers::WIPOffset<flatbuffers::Vector<'b , flatbuffers::ForwardsUOffset<ObjectGenerationRange<'b >>>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(ObjectGenerationRangeArray::VT_VALUES, values);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> ObjectGenerationRangeArrayBuilder<'a, 'b> {
    let start = _fbb.start_table();
    ObjectGenerationRangeArrayBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<ObjectGenerationRangeArray<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for ObjectGenerationRangeArray<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("ObjectGenerationRangeArray");
      ds.field("values", &self.values());
      ds.finish()
  }
}
#[inline]
/// Verifies that a buffer of bytes contains a `ObjectGenerationRangeArray`
/// and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_object_generation_range_array_unchecked`.
pub fn root_as_object_generation_range_array(buf: &[u8]) -> Result<ObjectGenerationRangeArray, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::root::<ObjectGenerationRangeArray>(buf)
}
#[inline]
/// Verifies that a buffer of bytes contains a size prefixed
/// `ObjectGenerationRangeArray` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `size_prefixed_root_as_object_generation_range_array_unchecked`.
pub fn size_prefixed_root_as_object_generation_range_array(buf: &[u8]) -> Result<ObjectGenerationRangeArray, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::size_prefixed_root::<ObjectGenerationRangeArray>(buf)
}
#[inline]
/// Verifies, with the given options, that a buffer of bytes
/// contains a `ObjectGenerationRangeArray` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_object_generation_range_array_unchecked`.
pub fn root_as_object_generation_range_array_with_opts<'b, 'o>(
  opts: &'o flatbuffers::VerifierOptions,
  buf: &'b [u8],
) -> Result<ObjectGenerationRangeArray<'b>, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::root_with_opts::<ObjectGenerationRangeArray<'b>>(opts, buf)
}
#[inline]
/// Verifies, with the given verifier options, that a buffer of
/// bytes contains a size prefixed `ObjectGenerationRangeArray` and returns
/// it. Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_object_generation_range_array_unchecked`.
pub fn size_prefixed_root_as_object_generation_range_array_with_opts<'b, 'o>(
  opts: &'o flatbuffers::VerifierOptions,
  buf: &'b [u8],
) -> Result<ObjectGenerationRangeArray<'b>, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::size_prefixed_root_with_opts::<ObjectGenerationRangeArray<'b>>(opts, buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a ObjectGenerationRangeArray and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid `ObjectGenerationRangeArray`.
pub unsafe fn root_as_object_generation_range_array_unchecked(buf: &[u8]) -> ObjectGenerationRangeArray {
  flatbuffers::root_unchecked::<ObjectGenerationRangeArray>(buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a size prefixed ObjectGenerationRangeArray and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid size prefixed `ObjectGenerationRangeArray`.
pub unsafe fn size_prefixed_root_as_object_generation_range_array_unchecked(buf: &[u8]) -> ObjectGenerationRangeArray {
  flatbuffers::size_prefixed_root_unchecked::<ObjectGenerationRangeArray>(buf)
}
#[inline]
pub fn finish_object_generation_range_array_buffer<'a, 'b>(
    fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    root: flatbuffers::WIPOffset<ObjectGenerationRangeArray<'a>>) {
  fbb.finish(root, None);
}

#[inline]
pub fn finish_size_prefixed_object_generation_range_array_buffer<'a, 'b>(fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>, root: flatbuffers::WIPOffset<ObjectGenerationRangeArray<'a>>) {
  fbb.finish_size_prefixed(root, None);
}
}