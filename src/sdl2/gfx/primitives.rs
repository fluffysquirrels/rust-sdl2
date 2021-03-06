//! Graphic Primitives

use std::mem;
use std::ptr;
use std::ffi::CString;
use num::traits::ToPrimitive;
use libc::{c_int, c_char};
use libc::c_void;
use render::Canvas;
use surface::Surface;
use pixels;
use crate::{Error, get_error_as_error};
use sys::gfx::primitives;

/// generic Color type
pub trait ToColor {
    fn as_rgba(&self) -> (u8, u8, u8, u8);

    #[inline]
    fn as_u32(&self) -> u32 {
        unsafe { mem::transmute(self.as_rgba()) }
    }
}

impl ToColor for pixels::Color {
    #[inline]
    fn as_rgba(&self) -> (u8, u8, u8, u8) {
        self.rgba()
    }
}

impl ToColor for (u8, u8, u8, u8) {
    #[inline]
    fn as_rgba(&self) -> (u8, u8, u8, u8) {
        *self
    }

    #[inline]
    fn as_u32(&self) -> u32 {
        unsafe { mem::transmute(*self) }
    }
}

impl ToColor for u32 {
    #[inline]
    fn as_rgba(&self) -> (u8, u8, u8, u8) {
        unsafe { mem::transmute(*self) }
    }

    #[inline]
    fn as_u32(&self) -> u32 {
        *self
    }
}

// for 0xXXXXXXXX
impl ToColor for isize {
    #[inline]
    fn as_rgba(&self) -> (u8, u8, u8, u8) {
        unsafe { mem::transmute(self.to_u32().expect("Can't convert to Color Type")) }
    }

    #[inline]
    fn as_u32(&self) -> u32 {
        self.to_u32().expect("Can't convert to Color Type")
    }
}

/// For drawing with rust-sdl2 Renderer
pub trait DrawRenderer {
    fn pixel<C: ToColor>(&self, x: i16, y: i16, color: C) -> Result<(), Error>;
    fn hline<C: ToColor>(&self, x1: i16, x2: i16, y: i16, color: C) -> Result<(), Error>;
    fn vline<C: ToColor>(&self, x: i16, y1: i16, y2: i16, color: C) -> Result<(), Error>;
    fn rectangle<C: ToColor>(&self,
                             x1: i16,
                             y1: i16,
                             x2: i16,
                             y2: i16,
                             color: C)
                             -> Result<(), Error>;
    fn rounded_rectangle<C: ToColor>(&self,
                                     x1: i16,
                                     y1: i16,
                                     x2: i16,
                                     y2: i16,
                                     rad: i16,
                                     color: C)
                                     -> Result<(), Error>;
    fn box_<C: ToColor>(&self, x1: i16, y1: i16, x2: i16, y2: i16, color: C) -> Result<(), Error>;
    fn rounded_box<C: ToColor>(&self,
                               x1: i16,
                               y1: i16,
                               x2: i16,
                               y2: i16,
                               rad: i16,
                               color: C)
                               -> Result<(), Error>;
    fn line<C: ToColor>(&self, x1: i16, y1: i16, x2: i16, y2: i16, color: C) -> Result<(), Error>;
    fn aa_line<C: ToColor>(&self,
                           x1: i16,
                           y1: i16,
                           x2: i16,
                           y2: i16,
                           color: C)
                           -> Result<(), Error>;
    fn thick_line<C: ToColor>(&self,
                              x1: i16,
                              y1: i16,
                              x2: i16,
                              y2: i16,
                              width: u8,
                              color: C)
                              -> Result<(), Error>;
    fn circle<C: ToColor>(&self, x: i16, y: i16, rad: i16, color: C) -> Result<(), Error>;
    fn aa_circle<C: ToColor>(&self, x: i16, y: i16, rad: i16, color: C) -> Result<(), Error>;
    fn filled_circle<C: ToColor>(&self, x: i16, y: i16, rad: i16, color: C) -> Result<(), Error>;
    fn arc<C: ToColor>(&self,
                       x: i16,
                       y: i16,
                       rad: i16,
                       start: i16,
                       end: i16,
                       color: C)
                       -> Result<(), Error>;
    fn ellipse<C: ToColor>(&self,
                           x: i16,
                           y: i16,
                           rx: i16,
                           ry: i16,
                           color: C)
                           -> Result<(), Error>;
    fn aa_ellipse<C: ToColor>(&self,
                              x: i16,
                              y: i16,
                              rx: i16,
                              ry: i16,
                              color: C)
                              -> Result<(), Error>;
    fn filled_ellipse<C: ToColor>(&self,
                                  x: i16,
                                  y: i16,
                                  rx: i16,
                                  ry: i16,
                                  color: C)
                                  -> Result<(), Error>;
    fn pie<C: ToColor>(&self,
                       x: i16,
                       y: i16,
                       rad: i16,
                       start: i16,
                       end: i16,
                       color: C)
                       -> Result<(), Error>;
    fn filled_pie<C: ToColor>(&self,
                              x: i16,
                              y: i16,
                              rad: i16,
                              start: i16,
                              end: i16,
                              color: C)
                              -> Result<(), Error>;
    fn trigon<C: ToColor>(&self,
                          x1: i16,
                          y1: i16,
                          x2: i16,
                          y2: i16,
                          x3: i16,
                          y3: i16,
                          color: C)
                          -> Result<(), Error>;
    fn aa_trigon<C: ToColor>(&self,
                             x1: i16,
                             y1: i16,
                             x2: i16,
                             y2: i16,
                             x3: i16,
                             y3: i16,
                             color: C)
                             -> Result<(), Error>;
    fn filled_trigon<C: ToColor>(&self,
                                 x1: i16,
                                 y1: i16,
                                 x2: i16,
                                 y2: i16,
                                 x3: i16,
                                 y3: i16,
                                 color: C)
                                 -> Result<(), Error>;
    fn polygon<C: ToColor>(&self, vx: &[i16], vy: &[i16], color: C) -> Result<(), Error>;
    fn aa_polygon<C: ToColor>(&self, vx: &[i16], vy: &[i16], color: C) -> Result<(), Error>;
    fn filled_polygon<C: ToColor>(&self, vx: &[i16], vy: &[i16], color: C) -> Result<(), Error>;
    fn textured_polygon<C: ToColor>(&self,
                                    vx: &[i16],
                                    vy: &[i16],
                                    texture: &Surface,
                                    texture_dx: i16,
                                    texture_dy: i16,
                                    color: C)
                                    -> Result<(), Error>;
    fn bezier<C: ToColor>(&self, vx: &[i16], vy: &[i16], s: i32, color: C) -> Result<(), Error>;
    fn character<C: ToColor>(&self, x: i16, y: i16, c: char, color: C) -> Result<(), Error>;
    fn string<C: ToColor>(&self, x: i16, y: i16, s: &str, color: C) -> Result<(), Error>;
}

impl<T> DrawRenderer for Canvas<T> where T: ::render::RenderTarget {
    fn pixel<C: ToColor>(&self, x: i16, y: i16, color: C) -> Result<(), Error> {
        let ret = unsafe { primitives::pixelColor(self.raw(), x, y, color.as_u32()) };
        if ret == 0 { Ok(()) } else { Err(get_error_as_error()) }
    }
    fn hline<C: ToColor>(&self, x1: i16, x2: i16, y: i16, color: C) -> Result<(), Error> {
        let ret = unsafe { primitives::hlineColor(self.raw(), x1, x2, y, color.as_u32()) };
        if ret == 0 { Ok(()) } else { Err(get_error_as_error()) }
    }
    fn vline<C: ToColor>(&self, x: i16, y1: i16, y2: i16, color: C) -> Result<(), Error> {
        let ret = unsafe { primitives::vlineColor(self.raw(), x, y1, y2, color.as_u32()) };
        if ret == 0 { Ok(()) } else { Err(get_error_as_error()) }
    }
    fn rectangle<C: ToColor>(&self,
                             x1: i16,
                             y1: i16,
                             x2: i16,
                             y2: i16,
                             color: C)
                             -> Result<(), Error> {
        let ret = unsafe { primitives::rectangleColor(self.raw(), x1, y1, x2, y2, color.as_u32()) };
        if ret == 0 { Ok(()) } else { Err(get_error_as_error()) }
    }
    fn rounded_rectangle<C: ToColor>(&self,
                                     x1: i16,
                                     y1: i16,
                                     x2: i16,
                                     y2: i16,
                                     rad: i16,
                                     color: C)
                                     -> Result<(), Error> {
        let ret =
            unsafe { primitives::roundedRectangleColor(self.raw(), x1, y1, x2, y2, rad, color.as_u32()) };
        if ret == 0 { Ok(()) } else { Err(get_error_as_error()) }
    }
    fn box_<C: ToColor>(&self, x1: i16, y1: i16, x2: i16, y2: i16, color: C) -> Result<(), Error> {
        let ret = unsafe { primitives::boxColor(self.raw(), x1, y1, x2, y2, color.as_u32()) };
        if ret == 0 { Ok(()) } else { Err(get_error_as_error()) }
    }
    fn rounded_box<C: ToColor>(&self,
                               x1: i16,
                               y1: i16,
                               x2: i16,
                               y2: i16,
                               rad: i16,
                               color: C)
                               -> Result<(), Error> {
        let ret = unsafe { primitives::roundedBoxColor(self.raw(), x1, y1, x2, y2, rad, color.as_u32()) };
        if ret == 0 { Ok(()) } else { Err(get_error_as_error()) }
    }
    fn line<C: ToColor>(&self, x1: i16, y1: i16, x2: i16, y2: i16, color: C) -> Result<(), Error> {
        let ret = unsafe { primitives::lineColor(self.raw(), x1, y1, x2, y2, color.as_u32()) };
        if ret == 0 { Ok(()) } else { Err(get_error_as_error()) }
    }
    fn aa_line<C: ToColor>(&self,
                           x1: i16,
                           y1: i16,
                           x2: i16,
                           y2: i16,
                           color: C)
                           -> Result<(), Error> {
        let ret = unsafe { primitives::aalineColor(self.raw(), x1, y1, x2, y2, color.as_u32()) };
        if ret == 0 { Ok(()) } else { Err(get_error_as_error()) }
    }
    fn thick_line<C: ToColor>(&self,
                              x1: i16,
                              y1: i16,
                              x2: i16,
                              y2: i16,
                              width: u8,
                              color: C)
                              -> Result<(), Error> {
        let ret = unsafe { primitives::thickLineColor(self.raw(), x1, y1, x2, y2, width, color.as_u32()) };
        if ret == 0 { Ok(()) } else { Err(get_error_as_error()) }
    }
    fn circle<C: ToColor>(&self, x: i16, y: i16, rad: i16, color: C) -> Result<(), Error> {
        let ret = unsafe { primitives::circleColor(self.raw(), x, y, rad, color.as_u32()) };
        if ret == 0 { Ok(()) } else { Err(get_error_as_error()) }
    }
    fn aa_circle<C: ToColor>(&self, x: i16, y: i16, rad: i16, color: C) -> Result<(), Error> {
        let ret = unsafe { primitives::aacircleColor(self.raw(), x, y, rad, color.as_u32()) };
        if ret == 0 { Ok(()) } else { Err(get_error_as_error()) }
    }
    fn filled_circle<C: ToColor>(&self, x: i16, y: i16, rad: i16, color: C) -> Result<(), Error> {
        let ret = unsafe { primitives::filledCircleColor(self.raw(), x, y, rad, color.as_u32()) };
        if ret == 0 { Ok(()) } else { Err(get_error_as_error()) }
    }
    fn arc<C: ToColor>(&self,
                       x: i16,
                       y: i16,
                       rad: i16,
                       start: i16,
                       end: i16,
                       color: C)
                       -> Result<(), Error> {
        let ret = unsafe { primitives::arcColor(self.raw(), x, y, rad, start, end, color.as_u32()) };
        if ret == 0 { Ok(()) } else { Err(get_error_as_error()) }
    }
    fn ellipse<C: ToColor>(&self,
                           x: i16,
                           y: i16,
                           rx: i16,
                           ry: i16,
                           color: C)
                           -> Result<(), Error> {
        let ret = unsafe { primitives::ellipseColor(self.raw(), x, y, rx, ry, color.as_u32()) };
        if ret == 0 { Ok(()) } else { Err(get_error_as_error()) }
    }
    fn aa_ellipse<C: ToColor>(&self,
                              x: i16,
                              y: i16,
                              rx: i16,
                              ry: i16,
                              color: C)
                              -> Result<(), Error> {
        let ret = unsafe { primitives::aaellipseColor(self.raw(), x, y, rx, ry, color.as_u32()) };
        if ret == 0 { Ok(()) } else { Err(get_error_as_error()) }
    }
    fn filled_ellipse<C: ToColor>(&self,
                                  x: i16,
                                  y: i16,
                                  rx: i16,
                                  ry: i16,
                                  color: C)
                                  -> Result<(), Error> {
        let ret = unsafe { primitives::filledEllipseColor(self.raw(), x, y, rx, ry, color.as_u32()) };
        if ret == 0 { Ok(()) } else { Err(get_error_as_error()) }
    }
    fn pie<C: ToColor>(&self,
                       x: i16,
                       y: i16,
                       rad: i16,
                       start: i16,
                       end: i16,
                       color: C)
                       -> Result<(), Error> {
        let ret = unsafe { primitives::pieColor(self.raw(), x, y, rad, start, end, color.as_u32()) };
        if ret == 0 { Ok(()) } else { Err(get_error_as_error()) }
    }
    fn filled_pie<C: ToColor>(&self,
                              x: i16,
                              y: i16,
                              rad: i16,
                              start: i16,
                              end: i16,
                              color: C)
                              -> Result<(), Error> {
        let ret = unsafe { primitives::filledPieColor(self.raw(), x, y, rad, start, end, color.as_u32()) };
        if ret == 0 { Ok(()) } else { Err(get_error_as_error()) }
    }
    fn trigon<C: ToColor>(&self,
                          x1: i16,
                          y1: i16,
                          x2: i16,
                          y2: i16,
                          x3: i16,
                          y3: i16,
                          color: C)
                          -> Result<(), Error> {
        let ret = unsafe { primitives::trigonColor(self.raw(), x1, y1, x2, y2, x3, y3, color.as_u32()) };
        if ret == 0 { Ok(()) } else { Err(get_error_as_error()) }
    }
    fn aa_trigon<C: ToColor>(&self,
                             x1: i16,
                             y1: i16,
                             x2: i16,
                             y2: i16,
                             x3: i16,
                             y3: i16,
                             color: C)
                             -> Result<(), Error> {
        let ret = unsafe { primitives::aatrigonColor(self.raw(), x1, y1, x2, y2, x3, y3, color.as_u32()) };
        if ret == 0 { Ok(()) } else { Err(get_error_as_error()) }
    }
    fn filled_trigon<C: ToColor>(&self,
                                 x1: i16,
                                 y1: i16,
                                 x2: i16,
                                 y2: i16,
                                 x3: i16,
                                 y3: i16,
                                 color: C)
                                 -> Result<(), Error> {
        let ret =
            unsafe { primitives::filledTrigonColor(self.raw(), x1, y1, x2, y2, x3, y3, color.as_u32()) };
        if ret == 0 { Ok(()) } else { Err(get_error_as_error()) }
    }
    // FIXME: may we use pointer tuple?
    fn polygon<C: ToColor>(&self, vx: &[i16], vy: &[i16], color: C) -> Result<(), Error> {
        assert_eq!(vx.len(), vy.len());
        let n = vx.len() as c_int;
        let ret =
            unsafe { primitives::polygonColor(self.raw(), vx.as_ptr(), vy.as_ptr(), n, color.as_u32()) };
        if ret == 0 { Ok(()) } else { Err(get_error_as_error()) }
    }

    fn aa_polygon<C: ToColor>(&self, vx: &[i16], vy: &[i16], color: C) -> Result<(), Error> {
        assert_eq!(vx.len(), vy.len());
        let n = vx.len() as c_int;
        let ret =
            unsafe { primitives::aapolygonColor(self.raw(), vx.as_ptr(), vy.as_ptr(), n, color.as_u32()) };
        if ret == 0 { Ok(()) } else { Err(get_error_as_error()) }
    }

    fn filled_polygon<C: ToColor>(&self, vx: &[i16], vy: &[i16], color: C) -> Result<(), Error> {
        assert_eq!(vx.len(), vy.len());
        let n = vx.len() as c_int;
        let ret = unsafe {
            primitives::filledPolygonColor(self.raw(), vx.as_ptr(), vy.as_ptr(), n, color.as_u32())
        };
        if ret == 0 { Ok(()) } else { Err(get_error_as_error()) }
    }
    #[allow(unused_variables)]
    fn textured_polygon<C: ToColor>(&self,
                                    vx: &[i16],
                                    vy: &[i16],
                                    texture: &Surface,
                                    texture_dx: i16,
                                    texture_dy: i16,
                                    color: C)
                                    -> Result<(), Error> {
        unimplemented!()
    }

    fn bezier<C: ToColor>(&self, vx: &[i16], vy: &[i16], s: i32, color: C) -> Result<(), Error> {
        assert_eq!(vx.len(), vy.len());
        let n = vx.len() as c_int;
        let ret = unsafe {
            primitives::bezierColor(self.raw(),
                            vx.as_ptr(),
                            vy.as_ptr(),
                            n,
                            s as c_int,
                            color.as_u32())
        };
        if ret == 0 { Ok(()) } else { Err(get_error_as_error()) }
    }

    fn character<C: ToColor>(&self, x: i16, y: i16, c: char, color: C) -> Result<(), Error> {
        let ret = unsafe { primitives::characterColor(self.raw(), x, y, c as c_char, color.as_u32()) };
        if ret == 0 { Ok(()) } else { Err(get_error_as_error()) }
    }

    fn string<C: ToColor>(&self, x: i16, y: i16, s: &str, color: C) -> Result<(), Error> {
        let ret = unsafe {
            let cstring = CString::new(s).unwrap();
            let buf = cstring.as_bytes().as_ptr();
            primitives::stringColor(self.raw(), x, y, buf as *mut c_char, color.as_u32())
        };
        if ret == 0 { Ok(()) } else { Err(get_error_as_error()) }
    }
}

/// Sets or resets the current global font data.
pub fn set_font<'b, F>(fontdata: F, cw: u32, ch: u32)
    where F: Into<Option<&'b [u8]>>
{
    let actual_fontdata = match fontdata.into() {
        None => ptr::null(),
        Some(v) => v.as_ptr(),
    };
    unsafe { primitives::gfxPrimitivesSetFont(actual_fontdata as *const c_void, cw, ch) }
}

/// Sets current global font character rotation steps.
pub fn set_font_rotation(rotation: u32) {
    unsafe { primitives::gfxPrimitivesSetFontRotation(rotation as u32) }
}
