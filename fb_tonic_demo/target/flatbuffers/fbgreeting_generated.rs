// automatically generated by the FlatBuffers compiler, do not modify


// @generated

use core::mem;
use core::cmp::Ordering;

extern crate flatbuffers;
use self::flatbuffers::{EndianScalar, Follow};

#[allow(unused_imports, dead_code)]
pub mod fbdemo {

  use core::mem;
  use core::cmp::Ordering;

  extern crate flatbuffers;
  use self::flatbuffers::{EndianScalar, Follow};

pub enum GreetingsOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct Greetings<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Greetings<'a> {
  type Inner = Greetings<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> Greetings<'a> {
  pub const VT_NAME: flatbuffers::VOffsetT = 4;
  pub const VT_WORDS: flatbuffers::VOffsetT = 6;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    Greetings { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>,
    args: &'args GreetingsArgs<'args>
  ) -> flatbuffers::WIPOffset<Greetings<'bldr>> {
    let mut builder = GreetingsBuilder::new(_fbb);
    if let Some(x) = args.words { builder.add_words(x); }
    if let Some(x) = args.name { builder.add_name(x); }
    builder.finish()
  }


  #[inline]
  pub fn name(&self) -> Option<&'a str> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(Greetings::VT_NAME, None)}
  }
  #[inline]
  pub fn words(&self) -> Option<&'a str> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(Greetings::VT_WORDS, None)}
  }
}

impl flatbuffers::Verifiable for Greetings<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("name", Self::VT_NAME, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("words", Self::VT_WORDS, false)?
     .finish();
    Ok(())
  }
}
pub struct GreetingsArgs<'a> {
    pub name: Option<flatbuffers::WIPOffset<&'a str>>,
    pub words: Option<flatbuffers::WIPOffset<&'a str>>,
}
impl<'a> Default for GreetingsArgs<'a> {
  #[inline]
  fn default() -> Self {
    GreetingsArgs {
      name: None,
      words: None,
    }
  }
}

pub struct GreetingsBuilder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> GreetingsBuilder<'a, 'b, A> {
  #[inline]
  pub fn add_name(&mut self, name: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Greetings::VT_NAME, name);
  }
  #[inline]
  pub fn add_words(&mut self, words: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Greetings::VT_WORDS, words);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>) -> GreetingsBuilder<'a, 'b, A> {
    let start = _fbb.start_table();
    GreetingsBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Greetings<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for Greetings<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("Greetings");
      ds.field("name", &self.name());
      ds.field("words", &self.words());
      ds.finish()
  }
}
#[inline]
/// Verifies that a buffer of bytes contains a `Greetings`
/// and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_greetings_unchecked`.
pub fn root_as_greetings(buf: &[u8]) -> Result<Greetings, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::root::<Greetings>(buf)
}
#[inline]
/// Verifies that a buffer of bytes contains a size prefixed
/// `Greetings` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `size_prefixed_root_as_greetings_unchecked`.
pub fn size_prefixed_root_as_greetings(buf: &[u8]) -> Result<Greetings, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::size_prefixed_root::<Greetings>(buf)
}
#[inline]
/// Verifies, with the given options, that a buffer of bytes
/// contains a `Greetings` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_greetings_unchecked`.
pub fn root_as_greetings_with_opts<'b, 'o>(
  opts: &'o flatbuffers::VerifierOptions,
  buf: &'b [u8],
) -> Result<Greetings<'b>, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::root_with_opts::<Greetings<'b>>(opts, buf)
}
#[inline]
/// Verifies, with the given verifier options, that a buffer of
/// bytes contains a size prefixed `Greetings` and returns
/// it. Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_greetings_unchecked`.
pub fn size_prefixed_root_as_greetings_with_opts<'b, 'o>(
  opts: &'o flatbuffers::VerifierOptions,
  buf: &'b [u8],
) -> Result<Greetings<'b>, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::size_prefixed_root_with_opts::<Greetings<'b>>(opts, buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a Greetings and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid `Greetings`.
pub unsafe fn root_as_greetings_unchecked(buf: &[u8]) -> Greetings {
  flatbuffers::root_unchecked::<Greetings>(buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a size prefixed Greetings and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid size prefixed `Greetings`.
pub unsafe fn size_prefixed_root_as_greetings_unchecked(buf: &[u8]) -> Greetings {
  flatbuffers::size_prefixed_root_unchecked::<Greetings>(buf)
}
#[inline]
pub fn finish_greetings_buffer<'a, 'b, A: flatbuffers::Allocator + 'a>(
    fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
    root: flatbuffers::WIPOffset<Greetings<'a>>) {
  fbb.finish(root, None);
}

#[inline]
pub fn finish_size_prefixed_greetings_buffer<'a, 'b, A: flatbuffers::Allocator + 'a>(fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>, root: flatbuffers::WIPOffset<Greetings<'a>>) {
  fbb.finish_size_prefixed(root, None);
}
}  // pub mod FBDemo
