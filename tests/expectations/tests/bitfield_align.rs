/* automatically generated by rust-bindgen */


#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]


#[repr(C)]
#[derive(Debug, Default, Copy)]
pub struct A {
    pub x: ::std::os::raw::c_uchar,
    pub _bitfield_1: [u8; 2usize],
    pub y: ::std::os::raw::c_uchar,
    pub __bindgen_align: [u32; 0usize],
}
#[test]
fn bindgen_test_layout_A() {
    assert_eq!(::std::mem::size_of::<A>() , 4usize , concat ! (
               "Size of: " , stringify ! ( A ) ));
    assert_eq! (::std::mem::align_of::<A>() , 4usize , concat ! (
                "Alignment of " , stringify ! ( A ) ));
    assert_eq! (unsafe { & ( * ( 0 as * const A ) ) . x as * const _ as usize
                } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( A ) , "::" , stringify
                ! ( x ) ));
    assert_eq! (unsafe { & ( * ( 0 as * const A ) ) . y as * const _ as usize
                } , 3usize , concat ! (
                "Alignment of field: " , stringify ! ( A ) , "::" , stringify
                ! ( y ) ));
}
impl Clone for A {
    fn clone(&self) -> Self { *self }
}
impl A {
    #[inline]
    pub fn b1(&self) -> ::std::os::raw::c_uint {
        let mask = 1usize as u16;
        let unit_field_val: u16 =
            unsafe { ::std::mem::transmute(self._bitfield_1) };
        let val = (unit_field_val & mask) >> 0usize;
        unsafe { ::std::mem::transmute(val as u32) }
    }
    #[inline]
    pub fn set_b1(&mut self, val: ::std::os::raw::c_uint) {
        let mask = 1usize as u16;
        let val = val as u32 as u16;
        let mut unit_field_val: u16 =
            unsafe { ::std::mem::transmute(self._bitfield_1) };
        unit_field_val &= !mask;
        unit_field_val |= (val << 0usize) & mask;
        self._bitfield_1 = unsafe { ::std::mem::transmute(unit_field_val) };
    }
    #[inline]
    pub fn b2(&self) -> ::std::os::raw::c_uint {
        let mask = 2usize as u16;
        let unit_field_val: u16 =
            unsafe { ::std::mem::transmute(self._bitfield_1) };
        let val = (unit_field_val & mask) >> 1usize;
        unsafe { ::std::mem::transmute(val as u32) }
    }
    #[inline]
    pub fn set_b2(&mut self, val: ::std::os::raw::c_uint) {
        let mask = 2usize as u16;
        let val = val as u32 as u16;
        let mut unit_field_val: u16 =
            unsafe { ::std::mem::transmute(self._bitfield_1) };
        unit_field_val &= !mask;
        unit_field_val |= (val << 1usize) & mask;
        self._bitfield_1 = unsafe { ::std::mem::transmute(unit_field_val) };
    }
    #[inline]
    pub fn b3(&self) -> ::std::os::raw::c_uint {
        let mask = 4usize as u16;
        let unit_field_val: u16 =
            unsafe { ::std::mem::transmute(self._bitfield_1) };
        let val = (unit_field_val & mask) >> 2usize;
        unsafe { ::std::mem::transmute(val as u32) }
    }
    #[inline]
    pub fn set_b3(&mut self, val: ::std::os::raw::c_uint) {
        let mask = 4usize as u16;
        let val = val as u32 as u16;
        let mut unit_field_val: u16 =
            unsafe { ::std::mem::transmute(self._bitfield_1) };
        unit_field_val &= !mask;
        unit_field_val |= (val << 2usize) & mask;
        self._bitfield_1 = unsafe { ::std::mem::transmute(unit_field_val) };
    }
    #[inline]
    pub fn b4(&self) -> ::std::os::raw::c_uint {
        let mask = 8usize as u16;
        let unit_field_val: u16 =
            unsafe { ::std::mem::transmute(self._bitfield_1) };
        let val = (unit_field_val & mask) >> 3usize;
        unsafe { ::std::mem::transmute(val as u32) }
    }
    #[inline]
    pub fn set_b4(&mut self, val: ::std::os::raw::c_uint) {
        let mask = 8usize as u16;
        let val = val as u32 as u16;
        let mut unit_field_val: u16 =
            unsafe { ::std::mem::transmute(self._bitfield_1) };
        unit_field_val &= !mask;
        unit_field_val |= (val << 3usize) & mask;
        self._bitfield_1 = unsafe { ::std::mem::transmute(unit_field_val) };
    }
    #[inline]
    pub fn b5(&self) -> ::std::os::raw::c_uint {
        let mask = 16usize as u16;
        let unit_field_val: u16 =
            unsafe { ::std::mem::transmute(self._bitfield_1) };
        let val = (unit_field_val & mask) >> 4usize;
        unsafe { ::std::mem::transmute(val as u32) }
    }
    #[inline]
    pub fn set_b5(&mut self, val: ::std::os::raw::c_uint) {
        let mask = 16usize as u16;
        let val = val as u32 as u16;
        let mut unit_field_val: u16 =
            unsafe { ::std::mem::transmute(self._bitfield_1) };
        unit_field_val &= !mask;
        unit_field_val |= (val << 4usize) & mask;
        self._bitfield_1 = unsafe { ::std::mem::transmute(unit_field_val) };
    }
    #[inline]
    pub fn b6(&self) -> ::std::os::raw::c_uint {
        let mask = 32usize as u16;
        let unit_field_val: u16 =
            unsafe { ::std::mem::transmute(self._bitfield_1) };
        let val = (unit_field_val & mask) >> 5usize;
        unsafe { ::std::mem::transmute(val as u32) }
    }
    #[inline]
    pub fn set_b6(&mut self, val: ::std::os::raw::c_uint) {
        let mask = 32usize as u16;
        let val = val as u32 as u16;
        let mut unit_field_val: u16 =
            unsafe { ::std::mem::transmute(self._bitfield_1) };
        unit_field_val &= !mask;
        unit_field_val |= (val << 5usize) & mask;
        self._bitfield_1 = unsafe { ::std::mem::transmute(unit_field_val) };
    }
    #[inline]
    pub fn b7(&self) -> ::std::os::raw::c_uint {
        let mask = 64usize as u16;
        let unit_field_val: u16 =
            unsafe { ::std::mem::transmute(self._bitfield_1) };
        let val = (unit_field_val & mask) >> 6usize;
        unsafe { ::std::mem::transmute(val as u32) }
    }
    #[inline]
    pub fn set_b7(&mut self, val: ::std::os::raw::c_uint) {
        let mask = 64usize as u16;
        let val = val as u32 as u16;
        let mut unit_field_val: u16 =
            unsafe { ::std::mem::transmute(self._bitfield_1) };
        unit_field_val &= !mask;
        unit_field_val |= (val << 6usize) & mask;
        self._bitfield_1 = unsafe { ::std::mem::transmute(unit_field_val) };
    }
    #[inline]
    pub fn b8(&self) -> ::std::os::raw::c_uint {
        let mask = 128usize as u16;
        let unit_field_val: u16 =
            unsafe { ::std::mem::transmute(self._bitfield_1) };
        let val = (unit_field_val & mask) >> 7usize;
        unsafe { ::std::mem::transmute(val as u32) }
    }
    #[inline]
    pub fn set_b8(&mut self, val: ::std::os::raw::c_uint) {
        let mask = 128usize as u16;
        let val = val as u32 as u16;
        let mut unit_field_val: u16 =
            unsafe { ::std::mem::transmute(self._bitfield_1) };
        unit_field_val &= !mask;
        unit_field_val |= (val << 7usize) & mask;
        self._bitfield_1 = unsafe { ::std::mem::transmute(unit_field_val) };
    }
    #[inline]
    pub fn b9(&self) -> ::std::os::raw::c_uint {
        let mask = 256usize as u16;
        let unit_field_val: u16 =
            unsafe { ::std::mem::transmute(self._bitfield_1) };
        let val = (unit_field_val & mask) >> 8usize;
        unsafe { ::std::mem::transmute(val as u32) }
    }
    #[inline]
    pub fn set_b9(&mut self, val: ::std::os::raw::c_uint) {
        let mask = 256usize as u16;
        let val = val as u32 as u16;
        let mut unit_field_val: u16 =
            unsafe { ::std::mem::transmute(self._bitfield_1) };
        unit_field_val &= !mask;
        unit_field_val |= (val << 8usize) & mask;
        self._bitfield_1 = unsafe { ::std::mem::transmute(unit_field_val) };
    }
    #[inline]
    pub fn b10(&self) -> ::std::os::raw::c_uint {
        let mask = 512usize as u16;
        let unit_field_val: u16 =
            unsafe { ::std::mem::transmute(self._bitfield_1) };
        let val = (unit_field_val & mask) >> 9usize;
        unsafe { ::std::mem::transmute(val as u32) }
    }
    #[inline]
    pub fn set_b10(&mut self, val: ::std::os::raw::c_uint) {
        let mask = 512usize as u16;
        let val = val as u32 as u16;
        let mut unit_field_val: u16 =
            unsafe { ::std::mem::transmute(self._bitfield_1) };
        unit_field_val &= !mask;
        unit_field_val |= (val << 9usize) & mask;
        self._bitfield_1 = unsafe { ::std::mem::transmute(unit_field_val) };
    }
    #[inline]
    pub fn new_bitfield_1(b1: ::std::os::raw::c_uint,
                          b2: ::std::os::raw::c_uint,
                          b3: ::std::os::raw::c_uint,
                          b4: ::std::os::raw::c_uint,
                          b5: ::std::os::raw::c_uint,
                          b6: ::std::os::raw::c_uint,
                          b7: ::std::os::raw::c_uint,
                          b8: ::std::os::raw::c_uint,
                          b9: ::std::os::raw::c_uint,
                          b10: ::std::os::raw::c_uint) -> u16 {
        ({
             ({
                  ({
                       ({
                            ({
                                 ({
                                      ({
                                           ({
                                                ({
                                                     ({ 0 } |
                                                          ((b1 as u32 as u16)
                                                               << 0usize) &
                                                              (1usize as u16))
                                                 } |
                                                     ((b2 as u32 as u16) <<
                                                          1usize) &
                                                         (2usize as u16))
                                            } |
                                                ((b3 as u32 as u16) << 2usize)
                                                    & (4usize as u16))
                                       } |
                                           ((b4 as u32 as u16) << 3usize) &
                                               (8usize as u16))
                                  } |
                                      ((b5 as u32 as u16) << 4usize) &
                                          (16usize as u16))
                             } |
                                 ((b6 as u32 as u16) << 5usize) &
                                     (32usize as u16))
                        } | ((b7 as u32 as u16) << 6usize) & (64usize as u16))
                   } | ((b8 as u32 as u16) << 7usize) & (128usize as u16))
              } | ((b9 as u32 as u16) << 8usize) & (256usize as u16))
         } | ((b10 as u32 as u16) << 9usize) & (512usize as u16))
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy)]
pub struct B {
    pub _bitfield_1: u32,
    pub __bindgen_align: [u32; 0usize],
}
#[test]
fn bindgen_test_layout_B() {
    assert_eq!(::std::mem::size_of::<B>() , 4usize , concat ! (
               "Size of: " , stringify ! ( B ) ));
    assert_eq! (::std::mem::align_of::<B>() , 4usize , concat ! (
                "Alignment of " , stringify ! ( B ) ));
}
impl Clone for B {
    fn clone(&self) -> Self { *self }
}
impl B {
    #[inline]
    pub fn foo(&self) -> ::std::os::raw::c_uint {
        let mask = 2147483647usize as u32;
        let unit_field_val: u32 =
            unsafe { ::std::mem::transmute(self._bitfield_1) };
        let val = (unit_field_val & mask) >> 0usize;
        unsafe { ::std::mem::transmute(val as u32) }
    }
    #[inline]
    pub fn set_foo(&mut self, val: ::std::os::raw::c_uint) {
        let mask = 2147483647usize as u32;
        let val = val as u32 as u32;
        let mut unit_field_val: u32 =
            unsafe { ::std::mem::transmute(self._bitfield_1) };
        unit_field_val &= !mask;
        unit_field_val |= (val << 0usize) & mask;
        self._bitfield_1 = unsafe { ::std::mem::transmute(unit_field_val) };
    }
    #[inline]
    pub fn bar(&self) -> ::std::os::raw::c_uchar {
        let mask = 2147483648usize as u32;
        let unit_field_val: u32 =
            unsafe { ::std::mem::transmute(self._bitfield_1) };
        let val = (unit_field_val & mask) >> 31usize;
        unsafe { ::std::mem::transmute(val as u8) }
    }
    #[inline]
    pub fn set_bar(&mut self, val: ::std::os::raw::c_uchar) {
        let mask = 2147483648usize as u32;
        let val = val as u8 as u32;
        let mut unit_field_val: u32 =
            unsafe { ::std::mem::transmute(self._bitfield_1) };
        unit_field_val &= !mask;
        unit_field_val |= (val << 31usize) & mask;
        self._bitfield_1 = unsafe { ::std::mem::transmute(unit_field_val) };
    }
    #[inline]
    pub fn new_bitfield_1(foo: ::std::os::raw::c_uint,
                          bar: ::std::os::raw::c_uchar) -> u32 {
        ({
             ({ 0 } |
                  ((foo as u32 as u32) << 0usize) & (2147483647usize as u32))
         } | ((bar as u8 as u32) << 31usize) & (2147483648usize as u32))
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy)]
pub struct C {
    pub x: ::std::os::raw::c_uchar,
    pub _bitfield_1: u8,
    pub baz: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout_C() {
    assert_eq!(::std::mem::size_of::<C>() , 8usize , concat ! (
               "Size of: " , stringify ! ( C ) ));
    assert_eq! (::std::mem::align_of::<C>() , 4usize , concat ! (
                "Alignment of " , stringify ! ( C ) ));
    assert_eq! (unsafe { & ( * ( 0 as * const C ) ) . x as * const _ as usize
                } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( C ) , "::" , stringify
                ! ( x ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const C ) ) . baz as * const _ as usize } ,
                4usize , concat ! (
                "Alignment of field: " , stringify ! ( C ) , "::" , stringify
                ! ( baz ) ));
}
impl Clone for C {
    fn clone(&self) -> Self { *self }
}
impl C {
    #[inline]
    pub fn b1(&self) -> ::std::os::raw::c_uint {
        let mask = 1usize as u8;
        let unit_field_val: u8 =
            unsafe { ::std::mem::transmute(self._bitfield_1) };
        let val = (unit_field_val & mask) >> 0usize;
        unsafe { ::std::mem::transmute(val as u32) }
    }
    #[inline]
    pub fn set_b1(&mut self, val: ::std::os::raw::c_uint) {
        let mask = 1usize as u8;
        let val = val as u32 as u8;
        let mut unit_field_val: u8 =
            unsafe { ::std::mem::transmute(self._bitfield_1) };
        unit_field_val &= !mask;
        unit_field_val |= (val << 0usize) & mask;
        self._bitfield_1 = unsafe { ::std::mem::transmute(unit_field_val) };
    }
    #[inline]
    pub fn b2(&self) -> ::std::os::raw::c_uint {
        let mask = 2usize as u8;
        let unit_field_val: u8 =
            unsafe { ::std::mem::transmute(self._bitfield_1) };
        let val = (unit_field_val & mask) >> 1usize;
        unsafe { ::std::mem::transmute(val as u32) }
    }
    #[inline]
    pub fn set_b2(&mut self, val: ::std::os::raw::c_uint) {
        let mask = 2usize as u8;
        let val = val as u32 as u8;
        let mut unit_field_val: u8 =
            unsafe { ::std::mem::transmute(self._bitfield_1) };
        unit_field_val &= !mask;
        unit_field_val |= (val << 1usize) & mask;
        self._bitfield_1 = unsafe { ::std::mem::transmute(unit_field_val) };
    }
    #[inline]
    pub fn new_bitfield_1(b1: ::std::os::raw::c_uint,
                          b2: ::std::os::raw::c_uint) -> u8 {
        ({ ({ 0 } | ((b1 as u32 as u8) << 0usize) & (1usize as u8)) } |
             ((b2 as u32 as u8) << 1usize) & (2usize as u8))
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy)]
pub struct Date1 {
    pub _bitfield_1: [u8; 4usize],
    pub __bindgen_align: [u16; 0usize],
}
#[test]
fn bindgen_test_layout_Date1() {
    assert_eq!(::std::mem::size_of::<Date1>() , 4usize , concat ! (
               "Size of: " , stringify ! ( Date1 ) ));
    assert_eq! (::std::mem::align_of::<Date1>() , 2usize , concat ! (
                "Alignment of " , stringify ! ( Date1 ) ));
}
impl Clone for Date1 {
    fn clone(&self) -> Self { *self }
}
impl Date1 {
    #[inline]
    pub fn nWeekDay(&self) -> ::std::os::raw::c_ushort {
        let mask = 7usize as u32;
        let unit_field_val: u32 =
            unsafe { ::std::mem::transmute(self._bitfield_1) };
        let val = (unit_field_val & mask) >> 0usize;
        unsafe { ::std::mem::transmute(val as u16) }
    }
    #[inline]
    pub fn set_nWeekDay(&mut self, val: ::std::os::raw::c_ushort) {
        let mask = 7usize as u32;
        let val = val as u16 as u32;
        let mut unit_field_val: u32 =
            unsafe { ::std::mem::transmute(self._bitfield_1) };
        unit_field_val &= !mask;
        unit_field_val |= (val << 0usize) & mask;
        self._bitfield_1 = unsafe { ::std::mem::transmute(unit_field_val) };
    }
    #[inline]
    pub fn nMonthDay(&self) -> ::std::os::raw::c_ushort {
        let mask = 504usize as u32;
        let unit_field_val: u32 =
            unsafe { ::std::mem::transmute(self._bitfield_1) };
        let val = (unit_field_val & mask) >> 3usize;
        unsafe { ::std::mem::transmute(val as u16) }
    }
    #[inline]
    pub fn set_nMonthDay(&mut self, val: ::std::os::raw::c_ushort) {
        let mask = 504usize as u32;
        let val = val as u16 as u32;
        let mut unit_field_val: u32 =
            unsafe { ::std::mem::transmute(self._bitfield_1) };
        unit_field_val &= !mask;
        unit_field_val |= (val << 3usize) & mask;
        self._bitfield_1 = unsafe { ::std::mem::transmute(unit_field_val) };
    }
    #[inline]
    pub fn nMonth(&self) -> ::std::os::raw::c_ushort {
        let mask = 15872usize as u32;
        let unit_field_val: u32 =
            unsafe { ::std::mem::transmute(self._bitfield_1) };
        let val = (unit_field_val & mask) >> 9usize;
        unsafe { ::std::mem::transmute(val as u16) }
    }
    #[inline]
    pub fn set_nMonth(&mut self, val: ::std::os::raw::c_ushort) {
        let mask = 15872usize as u32;
        let val = val as u16 as u32;
        let mut unit_field_val: u32 =
            unsafe { ::std::mem::transmute(self._bitfield_1) };
        unit_field_val &= !mask;
        unit_field_val |= (val << 9usize) & mask;
        self._bitfield_1 = unsafe { ::std::mem::transmute(unit_field_val) };
    }
    #[inline]
    pub fn nYear(&self) -> ::std::os::raw::c_ushort {
        let mask = 16711680usize as u32;
        let unit_field_val: u32 =
            unsafe { ::std::mem::transmute(self._bitfield_1) };
        let val = (unit_field_val & mask) >> 16usize;
        unsafe { ::std::mem::transmute(val as u16) }
    }
    #[inline]
    pub fn set_nYear(&mut self, val: ::std::os::raw::c_ushort) {
        let mask = 16711680usize as u32;
        let val = val as u16 as u32;
        let mut unit_field_val: u32 =
            unsafe { ::std::mem::transmute(self._bitfield_1) };
        unit_field_val &= !mask;
        unit_field_val |= (val << 16usize) & mask;
        self._bitfield_1 = unsafe { ::std::mem::transmute(unit_field_val) };
    }
    #[inline]
    pub fn new_bitfield_1(nWeekDay: ::std::os::raw::c_ushort,
                          nMonthDay: ::std::os::raw::c_ushort,
                          nMonth: ::std::os::raw::c_ushort,
                          nYear: ::std::os::raw::c_ushort) -> u32 {
        ({
             ({
                  ({
                       ({ 0 } |
                            ((nWeekDay as u16 as u32) << 0usize) &
                                (7usize as u32))
                   } |
                       ((nMonthDay as u16 as u32) << 3usize) &
                           (504usize as u32))
              } | ((nMonth as u16 as u32) << 9usize) & (15872usize as u32))
         } | ((nYear as u16 as u32) << 16usize) & (16711680usize as u32))
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy)]
pub struct Date2 {
    pub _bitfield_1: [u8; 4usize],
    pub __bindgen_align: [u16; 0usize],
}
#[test]
fn bindgen_test_layout_Date2() {
    assert_eq!(::std::mem::size_of::<Date2>() , 4usize , concat ! (
               "Size of: " , stringify ! ( Date2 ) ));
    assert_eq! (::std::mem::align_of::<Date2>() , 2usize , concat ! (
                "Alignment of " , stringify ! ( Date2 ) ));
}
impl Clone for Date2 {
    fn clone(&self) -> Self { *self }
}
impl Date2 {
    #[inline]
    pub fn nWeekDay(&self) -> ::std::os::raw::c_ushort {
        let mask = 7usize as u32;
        let unit_field_val: u32 =
            unsafe { ::std::mem::transmute(self._bitfield_1) };
        let val = (unit_field_val & mask) >> 0usize;
        unsafe { ::std::mem::transmute(val as u16) }
    }
    #[inline]
    pub fn set_nWeekDay(&mut self, val: ::std::os::raw::c_ushort) {
        let mask = 7usize as u32;
        let val = val as u16 as u32;
        let mut unit_field_val: u32 =
            unsafe { ::std::mem::transmute(self._bitfield_1) };
        unit_field_val &= !mask;
        unit_field_val |= (val << 0usize) & mask;
        self._bitfield_1 = unsafe { ::std::mem::transmute(unit_field_val) };
    }
    #[inline]
    pub fn nMonthDay(&self) -> ::std::os::raw::c_ushort {
        let mask = 504usize as u32;
        let unit_field_val: u32 =
            unsafe { ::std::mem::transmute(self._bitfield_1) };
        let val = (unit_field_val & mask) >> 3usize;
        unsafe { ::std::mem::transmute(val as u16) }
    }
    #[inline]
    pub fn set_nMonthDay(&mut self, val: ::std::os::raw::c_ushort) {
        let mask = 504usize as u32;
        let val = val as u16 as u32;
        let mut unit_field_val: u32 =
            unsafe { ::std::mem::transmute(self._bitfield_1) };
        unit_field_val &= !mask;
        unit_field_val |= (val << 3usize) & mask;
        self._bitfield_1 = unsafe { ::std::mem::transmute(unit_field_val) };
    }
    #[inline]
    pub fn nMonth(&self) -> ::std::os::raw::c_ushort {
        let mask = 15872usize as u32;
        let unit_field_val: u32 =
            unsafe { ::std::mem::transmute(self._bitfield_1) };
        let val = (unit_field_val & mask) >> 9usize;
        unsafe { ::std::mem::transmute(val as u16) }
    }
    #[inline]
    pub fn set_nMonth(&mut self, val: ::std::os::raw::c_ushort) {
        let mask = 15872usize as u32;
        let val = val as u16 as u32;
        let mut unit_field_val: u32 =
            unsafe { ::std::mem::transmute(self._bitfield_1) };
        unit_field_val &= !mask;
        unit_field_val |= (val << 9usize) & mask;
        self._bitfield_1 = unsafe { ::std::mem::transmute(unit_field_val) };
    }
    #[inline]
    pub fn nYear(&self) -> ::std::os::raw::c_ushort {
        let mask = 16711680usize as u32;
        let unit_field_val: u32 =
            unsafe { ::std::mem::transmute(self._bitfield_1) };
        let val = (unit_field_val & mask) >> 16usize;
        unsafe { ::std::mem::transmute(val as u16) }
    }
    #[inline]
    pub fn set_nYear(&mut self, val: ::std::os::raw::c_ushort) {
        let mask = 16711680usize as u32;
        let val = val as u16 as u32;
        let mut unit_field_val: u32 =
            unsafe { ::std::mem::transmute(self._bitfield_1) };
        unit_field_val &= !mask;
        unit_field_val |= (val << 16usize) & mask;
        self._bitfield_1 = unsafe { ::std::mem::transmute(unit_field_val) };
    }
    #[inline]
    pub fn byte(&self) -> ::std::os::raw::c_uchar {
        let mask = 4278190080usize as u32;
        let unit_field_val: u32 =
            unsafe { ::std::mem::transmute(self._bitfield_1) };
        let val = (unit_field_val & mask) >> 24usize;
        unsafe { ::std::mem::transmute(val as u8) }
    }
    #[inline]
    pub fn set_byte(&mut self, val: ::std::os::raw::c_uchar) {
        let mask = 4278190080usize as u32;
        let val = val as u8 as u32;
        let mut unit_field_val: u32 =
            unsafe { ::std::mem::transmute(self._bitfield_1) };
        unit_field_val &= !mask;
        unit_field_val |= (val << 24usize) & mask;
        self._bitfield_1 = unsafe { ::std::mem::transmute(unit_field_val) };
    }
    #[inline]
    pub fn new_bitfield_1(nWeekDay: ::std::os::raw::c_ushort,
                          nMonthDay: ::std::os::raw::c_ushort,
                          nMonth: ::std::os::raw::c_ushort,
                          nYear: ::std::os::raw::c_ushort,
                          byte: ::std::os::raw::c_uchar) -> u32 {
        ({
             ({
                  ({
                       ({
                            ({ 0 } |
                                 ((nWeekDay as u16 as u32) << 0usize) &
                                     (7usize as u32))
                        } |
                            ((nMonthDay as u16 as u32) << 3usize) &
                                (504usize as u32))
                   } |
                       ((nMonth as u16 as u32) << 9usize) &
                           (15872usize as u32))
              } | ((nYear as u16 as u32) << 16usize) & (16711680usize as u32))
         } | ((byte as u8 as u32) << 24usize) & (4278190080usize as u32))
    }
}
