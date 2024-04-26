// automatically generated by the FlatBuffers compiler, do not modify


// @generated

use core::mem;
use core::cmp::Ordering;

extern crate flatbuffers;
use self::flatbuffers::{EndianScalar, Follow};

#[allow(unused_imports, dead_code)]
pub mod my_game {

  use core::mem;
  use core::cmp::Ordering;

  extern crate flatbuffers;
  use self::flatbuffers::{EndianScalar, Follow};
#[allow(unused_imports, dead_code)]
pub mod sample {

  use core::mem;
  use core::cmp::Ordering;

  extern crate flatbuffers;
  use self::flatbuffers::{EndianScalar, Follow};

pub enum MonsterOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct Monster<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Monster<'a> {
  type Inner = Monster<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> Monster<'a> {
  pub const VT_ID: flatbuffers::VOffsetT = 4;
  pub const VT_NAME: flatbuffers::VOffsetT = 6;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    Monster { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>,
    args: &'args MonsterArgs<'args>
  ) -> flatbuffers::WIPOffset<Monster<'bldr>> {
    let mut builder = MonsterBuilder::new(_fbb);
    if let Some(x) = args.name { builder.add_name(x); }
    builder.add_id(args.id);
    builder.finish()
  }


  #[inline]
  pub fn id(&self) -> i32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<i32>(Monster::VT_ID, Some(0)).unwrap()}
  }
  #[inline]
  pub fn name(&self) -> Option<&'a str> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(Monster::VT_NAME, None)}
  }
}

impl flatbuffers::Verifiable for Monster<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<i32>("id", Self::VT_ID, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("name", Self::VT_NAME, false)?
     .finish();
    Ok(())
  }
}
pub struct MonsterArgs<'a> {
    pub id: i32,
    pub name: Option<flatbuffers::WIPOffset<&'a str>>,
}
impl<'a> Default for MonsterArgs<'a> {
  #[inline]
  fn default() -> Self {
    MonsterArgs {
      id: 0,
      name: None,
    }
  }
}

pub struct MonsterBuilder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> MonsterBuilder<'a, 'b, A> {
  #[inline]
  pub fn add_id(&mut self, id: i32) {
    self.fbb_.push_slot::<i32>(Monster::VT_ID, id, 0);
  }
  #[inline]
  pub fn add_name(&mut self, name: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Monster::VT_NAME, name);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>) -> MonsterBuilder<'a, 'b, A> {
    let start = _fbb.start_table();
    MonsterBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Monster<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for Monster<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("Monster");
      ds.field("id", &self.id());
      ds.field("name", &self.name());
      ds.finish()
  }
}
}  // pub mod Sample
}  // pub mod MyGame
