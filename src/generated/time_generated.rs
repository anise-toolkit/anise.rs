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
pub mod time {

  use std::mem;
  use std::cmp::Ordering;

  extern crate flatbuffers;
  use self::flatbuffers::{EndianScalar, Follow};

#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MIN_SYSTEM: u8 = 0;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MAX_SYSTEM: u8 = 5;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
#[allow(non_camel_case_types)]
pub const ENUM_VALUES_SYSTEM: [System; 6] = [
  System::TDB,
  System::UTC,
  System::TAI,
  System::TT,
  System::GPS,
  System::SCLK,
];

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct System(pub u8);
#[allow(non_upper_case_globals)]
impl System {
  pub const TDB: Self = Self(0);
  pub const UTC: Self = Self(1);
  pub const TAI: Self = Self(2);
  pub const TT: Self = Self(3);
  pub const GPS: Self = Self(4);
  pub const SCLK: Self = Self(5);

  pub const ENUM_MIN: u8 = 0;
  pub const ENUM_MAX: u8 = 5;
  pub const ENUM_VALUES: &'static [Self] = &[
    Self::TDB,
    Self::UTC,
    Self::TAI,
    Self::TT,
    Self::GPS,
    Self::SCLK,
  ];
  /// Returns the variant's name or "" if unknown.
  pub fn variant_name(self) -> Option<&'static str> {
    match self {
      Self::TDB => Some("TDB"),
      Self::UTC => Some("UTC"),
      Self::TAI => Some("TAI"),
      Self::TT => Some("TT"),
      Self::GPS => Some("GPS"),
      Self::SCLK => Some("SCLK"),
      _ => None,
    }
  }
}
impl std::fmt::Debug for System {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    if let Some(name) = self.variant_name() {
      f.write_str(name)
    } else {
      f.write_fmt(format_args!("<UNKNOWN {:?}>", self.0))
    }
  }
}
impl<'a> flatbuffers::Follow<'a> for System {
  type Inner = Self;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    let b = unsafe {
      flatbuffers::read_scalar_at::<u8>(buf, loc)
    };
    Self(b)
  }
}

impl flatbuffers::Push for System {
    type Output = System;
    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        unsafe { flatbuffers::emplace_scalar::<u8>(dst, self.0); }
    }
}

impl flatbuffers::EndianScalar for System {
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

impl<'a> flatbuffers::Verifiable for System {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    u8::run_verifier(v, pos)
  }
}

impl flatbuffers::SimpleToVerifyInSlice for System {}
// struct Epoch, aligned to 8
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq)]
pub struct Epoch(pub [u8; 16]);
impl Default for Epoch { 
  fn default() -> Self { 
    Self([0; 16])
  }
}
impl std::fmt::Debug for Epoch {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    f.debug_struct("Epoch")
      .field("hi", &self.hi())
      .field("lo", &self.lo())
      .finish()
  }
}

impl flatbuffers::SimpleToVerifyInSlice for Epoch {}
impl flatbuffers::SafeSliceAccess for Epoch {}
impl<'a> flatbuffers::Follow<'a> for Epoch {
  type Inner = &'a Epoch;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    <&'a Epoch>::follow(buf, loc)
  }
}
impl<'a> flatbuffers::Follow<'a> for &'a Epoch {
  type Inner = &'a Epoch;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    flatbuffers::follow_cast_ref::<Epoch>(buf, loc)
  }
}
impl<'b> flatbuffers::Push for Epoch {
    type Output = Epoch;
    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        let src = unsafe {
            ::std::slice::from_raw_parts(self as *const Epoch as *const u8, Self::size())
        };
        dst.copy_from_slice(src);
    }
}
impl<'b> flatbuffers::Push for &'b Epoch {
    type Output = Epoch;

    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        let src = unsafe {
            ::std::slice::from_raw_parts(*self as *const Epoch as *const u8, Self::size())
        };
        dst.copy_from_slice(src);
    }
}

impl<'a> flatbuffers::Verifiable for Epoch {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.in_buffer::<Self>(pos)
  }
}
impl<'a> Epoch {
  #[allow(clippy::too_many_arguments)]
  pub fn new(
    hi: f64,
    lo: f64,
  ) -> Self {
    let mut s = Self([0; 16]);
    s.set_hi(hi);
    s.set_lo(lo);
    s
  }

  pub fn hi(&self) -> f64 {
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

  pub fn set_hi(&mut self, x: f64) {
    let x_le = x.to_little_endian();
    unsafe {
      core::ptr::copy_nonoverlapping(
        &x_le as *const f64 as *const u8,
        self.0[0..].as_mut_ptr(),
        core::mem::size_of::<f64>(),
      );
    }
  }

  pub fn lo(&self) -> f64 {
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

  pub fn set_lo(&mut self, x: f64) {
    let x_le = x.to_little_endian();
    unsafe {
      core::ptr::copy_nonoverlapping(
        &x_le as *const f64 as *const u8,
        self.0[8..].as_mut_ptr(),
        core::mem::size_of::<f64>(),
      );
    }
  }

}

}  // pub mod Time
}  // pub mod Anise

