use std::mem;
use rect::Rect;
use get_error;
use SdlResult;
use std::ptr;
use libc::c_int;
use pixels;
use rwops;

#[allow(non_camel_case_types)]
pub mod ll {
    use pixels::ll::SDL_PixelFormat;
    use pixels::ll::SDL_Palette;
    use render::ll::SDL_BlendMode;
    use rwops::ll::SDL_RWops;
    use rect::Rect;
    use libc::{c_int, c_void, uint32_t, uint8_t};
    pub type SDL_Rect = Rect;
    pub type SDL_bool = c_int;

    pub type SDL_SurfaceFlag = c_int;

    pub static SDL_SWSURFACE: SDL_SurfaceFlag = 0;
    pub static SDL_PREALLOC: SDL_SurfaceFlag = 0x00000001;
    pub static SDL_RLEACCEL: SDL_SurfaceFlag = 0x00000002;
    pub static SDL_DONTFREE: SDL_SurfaceFlag = 0x00000004;

    //SDL_surface.h
    pub struct SDL_BlitMap;

    pub struct SDL_Surface {
        pub flags: uint32_t,
        pub format: *const SDL_PixelFormat,
        pub w: c_int,
        pub h: c_int,
        pub pitch: c_int,
        pub pixels: *const c_void,
        pub userdata: *const c_void,
        pub locked: c_int,
        pub lock_data: *const c_void,
        pub clip_rect: SDL_Rect,
        pub map: *const SDL_BlitMap,
        pub refcount: c_int
    }

    extern "C" {
        pub fn SDL_CreateRGBSurface(flags: uint32_t, width: c_int, height: c_int, depth: c_int, Rmask: uint32_t, Gmask: uint32_t, Bmask: uint32_t, Amask: uint32_t) ->  *const SDL_Surface;
        pub fn SDL_CreateRGBSurfaceFrom(pixels: *const c_void, width: c_int, height: c_int, depth: c_int, pitch: c_int, Rmask: uint32_t, Gmask: uint32_t, Bmask: uint32_t, Amask: uint32_t) ->  *const SDL_Surface;
        pub fn SDL_FreeSurface(surface: *const SDL_Surface);
        pub fn SDL_SetSurfacePalette(surface: *const SDL_Surface, palette: *const SDL_Palette) -> c_int;
        pub fn SDL_LockSurface(surface: *const SDL_Surface) -> c_int;
        pub fn SDL_UnlockSurface(surface: *const SDL_Surface);
        pub fn SDL_LoadBMP_RW(src: *const SDL_RWops, freesrc: c_int) ->  *const SDL_Surface;
        pub fn SDL_SaveBMP_RW(surface: *const SDL_Surface, dst: *const SDL_RWops, freedst: c_int) -> c_int;
        pub fn SDL_SetSurfaceRLE(surface: *const SDL_Surface, flag: c_int) -> c_int;
        pub fn SDL_SetColorKey(surface: *const SDL_Surface, flag: c_int, key: uint32_t) -> c_int;
        pub fn SDL_GetColorKey(surface: *const SDL_Surface, key: *const uint32_t) -> c_int;
        pub fn SDL_SetSurfaceColorMod(surface: *const SDL_Surface, r: uint8_t, g: uint8_t, b: uint8_t) -> c_int;
        pub fn SDL_GetSurfaceColorMod(surface: *const SDL_Surface, r: *const uint8_t, g: *const uint8_t, b: *const uint8_t ) -> c_int;
        pub fn SDL_SetSurfaceAlphaMod(surface: *const SDL_Surface, alpha: uint8_t) -> c_int;
        pub fn SDL_GetSurfaceAlphaMod(surface: *const SDL_Surface, alpha: *const uint8_t ) -> c_int;
        pub fn SDL_SetSurfaceBlendMode(surface: *const SDL_Surface, blendMode: SDL_BlendMode) -> c_int;
        pub fn SDL_GetSurfaceBlendMode(surface: *const SDL_Surface, blendMode: *const SDL_BlendMode) -> c_int;
        pub fn SDL_SetClipRect(surface: *const SDL_Surface, rect: *const SDL_Rect) ->  SDL_bool;
        pub fn SDL_GetClipRect(surface: *const SDL_Surface, rect: *const SDL_Rect);
        pub fn SDL_ConvertSurface(src: *const SDL_Surface, fmt: *const SDL_PixelFormat, flags: uint32_t) ->  *const SDL_Surface;
        pub fn SDL_ConvertSurfaceFormat(src: *const SDL_Surface, pixel_format: uint32_t, flags: uint32_t) ->  *const SDL_Surface;
        pub fn SDL_ConvertPixels(width: c_int, height: c_int, src_format: uint32_t, src: *const c_void, src_pitch: c_int, dst_format: uint32_t, dst: *const c_void, dst_pitch: c_int) -> c_int;
        pub fn SDL_FillRect(dst: *const SDL_Surface, rect: *const SDL_Rect, color: uint32_t) -> c_int;
        pub fn SDL_FillRects(dst: *const SDL_Surface, rects: *const SDL_Rect, count: c_int, color: uint32_t) -> c_int;
        pub fn SDL_UpperBlit(src: *const SDL_Surface, srcrect: *const SDL_Rect, dst: *const SDL_Surface, dstrect: *const SDL_Rect) -> c_int;
        pub fn SDL_LowerBlit(src: *const SDL_Surface, srcrect: *const SDL_Rect, dst: *const SDL_Surface, dstrect: *const SDL_Rect) -> c_int;
        pub fn SDL_SoftStretch(src: *const SDL_Surface, srcrect: *const SDL_Rect, dst: *const SDL_Surface, dstrect: *const SDL_Rect) -> c_int;
        pub fn SDL_UpperBlitScaled(src: *const SDL_Surface, srcrect: *const SDL_Rect, dst: *const SDL_Surface, dstrect: *const SDL_Rect) -> c_int;
        pub fn SDL_LowerBlitScaled(src: *const SDL_Surface, srcrect: *const SDL_Rect, dst: *const SDL_Surface, dstrect: *const SDL_Rect) -> c_int;
    }
}

bitflags!(flags SurfaceFlag: u32 {
    static SWSurface = ll::SDL_SWSURFACE as u32,
    static PreAlloc = ll::SDL_PREALLOC as u32,
    static RLEAccel = ll::SDL_RLEACCEL as u32,
    static DontFree = ll::SDL_DONTFREE as u32
})

#[deriving(PartialEq)]
#[allow(raw_pointer_deriving)]
pub struct Surface {
    raw: *const ll::SDL_Surface,
    owned: bool
}

impl Drop for Surface {
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ll::SDL_FreeSurface(self.raw);
            }
        }
    }
}

impl_raw_accessors!(Surface, *const ll::SDL_Surface)
impl_owned_accessors!(Surface, owned)
impl_raw_constructor!(Surface -> Surface (raw: *const ll::SDL_Surface, owned: bool))

impl Surface {
    pub fn new(surface_flags: SurfaceFlag, width: int, height: int, bpp: int,
               rmask: u32, gmask: u32, bmask: u32, amask: u32) -> SdlResult<Surface> {
        unsafe {
            let raw = ll::SDL_CreateRGBSurface(surface_flags.bits(), width as c_int, height as c_int, bpp as c_int,
                                               rmask, gmask, bmask, amask);

            if raw == ptr::null() {
                Err(get_error())
            } else {
                Ok(Surface { raw: raw, owned: true })
            }
        }
    }

    //TODO: From Data
    pub fn get_width(&self) -> u16 {
        unsafe { (*self.raw).w as u16 }
    }

    pub fn get_height(&self) -> u16 {
        unsafe { (*self.raw).h as u16 }
    }

    pub fn get_size(&self) -> (u16, u16) {
        (self.get_width(), self.get_height())
    }

    pub fn get_rect(&self) -> Rect {
        Rect {
            x: 0,
            y: 0,
            w: self.get_width() as i32,
            h: self.get_height() as i32
        }
    }

    pub fn get_pixel_format(&self) -> pixels::PixelFormat {
        unsafe {
            pixels::PixelFormat::from_ll((*self.raw).format)
        }
    }

    pub fn lock(&self) -> bool {
        unsafe { ll::SDL_LockSurface(self.raw) == 0 }
    }

    /// Locks a surface so that the pixels can be directly accessed safely.
    pub fn with_lock<R>(&self, f: |pixels: &mut [u8]| -> R) -> R {
        unsafe {
            if ll::SDL_LockSurface(self.raw) != 0 { fail!("could not lock surface"); }
            let len = (*self.raw).pitch as uint * ((*self.raw).h as uint);
            let pixels: &mut [u8] = mem::transmute(((*self.raw).pixels, len));
            let rv = f(pixels);
            ll::SDL_UnlockSurface(self.raw);
            rv
        }
    }

    pub fn unlock(&self) {
        unsafe { ll::SDL_UnlockSurface(self.raw); }
    }

    pub fn from_bmp(path: &Path) -> SdlResult<Surface> {
        let raw = unsafe {
            ll::SDL_LoadBMP_RW(try!(rwops::RWops::from_file(path, "rb")).raw(), 0)
        };

        if raw.is_null() { Err(get_error()) }
        else { Ok(Surface{raw: raw, owned: true}) }
    }

    pub fn save_bmp(&self, path: &Path) -> SdlResult<()> {
	let ret = unsafe {
            ll::SDL_SaveBMP_RW(self.raw, try!(rwops::RWops::from_file(path, "rb")).raw(), 0)
	};
        if ret == 0 { Ok(()) }
        else { Err(get_error()) }
    }

    pub fn set_palette(&self, palette: &pixels::Palette) -> bool {
        unsafe {
            ll::SDL_SetSurfacePalette(self.raw, palette.raw()) == 0
        }
    }

    #[allow(non_snake_case_functions)]
    pub fn enable_RLE(&self) -> bool {
        unsafe {
            ll::SDL_SetSurfaceRLE(self.raw, 1) == 0
        }
    }

    #[allow(non_snake_case_functions)]
    pub fn disable_RLE(&self) -> bool {
        unsafe {
            ll::SDL_SetSurfaceRLE(self.raw, 0) == 0
        }
    }

    pub fn set_color_key(&self, enable: bool, color: pixels::Color) -> SdlResult<()> {
        let key = color.to_u32(&self.get_pixel_format());
        let result = unsafe {
            ll::SDL_SetColorKey(self.raw, if enable { 1 } else { 0 }, key)
        };
        if result == 0 {
            Ok(())
        } else {
            Err(get_error())
        }
    }

    pub fn get_color_key(&self) -> SdlResult<pixels::Color> {
        let key: u32 = 0;
        let result = unsafe {
            ll::SDL_GetColorKey(self.raw, &key)
        };

        if result == 0 {
            Ok(pixels::Color::from_u32(&self.get_pixel_format(), key))
        } else {
            Err(get_error())
        }
    }

    pub fn set_color_mod(&self, color: pixels::Color) -> bool {
        let (r, g, b) = match color {
            pixels::RGB(r, g, b) => (r, g, b),
            pixels::RGBA(r, g, b, _) => (r, g, b)
        };

        unsafe {
            ll::SDL_SetSurfaceColorMod(self.raw, r, g, b) == 0
        }
    }

    pub fn get_color_mod(&self) -> SdlResult<pixels::Color> {
        let r: u8 = 0;
        let g: u8 = 0;
        let b: u8 = 0;

        let result = unsafe {
            ll::SDL_GetSurfaceColorMod(self.raw, &r, &g, &b) == 0
        };

        if result {
            Ok(pixels::RGB(r,g,b))
        } else {
            Err(get_error())
        }
    }

    pub fn blit( &self, src: &Surface, dstrect: Option<Rect>, srcrect: Option<Rect> ) -> bool {
        unsafe {
            let dstrect_ptr = mem::transmute( dstrect.as_ref() );
            let srcrect_ptr = mem::transmute( srcrect.as_ref() );
            ll::SDL_UpperBlit( src.raw, srcrect_ptr, self.raw, dstrect_ptr ) == 0
        }
    }

    /*
    pub fn SDL_SetSurfaceAlphaMod(surface: *SDL_Surface, alpha: uint8_t) -> c_int;
    pub fn SDL_GetSurfaceAlphaMod(surface: *SDL_Surface, alpha: *uint8_t ) -> c_int;
    pub fn SDL_SetSurfaceBlendMode(surface: *SDL_Surface, blendMode: SDL_BlendMode) -> c_int;
    pub fn SDL_GetSurfaceBlendMode(surface: *SDL_Surface, blendMode: *SDL_BlendMode) -> c_int;
    pub fn SDL_SetClipRect(surface: *SDL_Surface, rect: *SDL_Rect) ->  SDL_bool;
    pub fn SDL_GetClipRect(surface: *SDL_Surface, rect: *SDL_Rect);
    pub fn SDL_ConvertSurface(src: *SDL_Surface, fmt: *SDL_PixelFormat, flags: uint32_t) ->  *SDL_Surface;
    pub fn SDL_ConvertSurfaceFormat(src: *SDL_Surface, pixel_format: uint32_t, flags: uint32_t) ->  *SDL_Surface;
    pub fn SDL_ConvertPixels(width: c_int, height: c_int, src_format: uint32_t, src: *c_void, src_pitch: c_int, dst_format: uint32_t, dst: *c_void, dst_pitch: c_int) -> c_int;
    pub fn SDL_FillRect(dst: *SDL_Surface, rect: *SDL_Rect, color: uint32_t) -> c_int;
    pub fn SDL_FillRects(dst: *SDL_Surface, rects: *SDL_Rect, count: c_int, color: uint32_t) -> c_int;
    pub fn SDL_LowerBlit(src: *SDL_Surface, srcrect: *SDL_Rect, dst: *SDL_Surface, dstrect: *SDL_Rect) -> c_int;
    pub fn SDL_SoftStretch(src: *SDL_Surface, srcrect: *SDL_Rect, dst: *SDL_Surface, dstrect: *SDL_Rect) -> c_int;
    pub fn SDL_UpperBlitScaled(src: *SDL_Surface, srcrect: *SDL_Rect, dst: *SDL_Surface, dstrect: *SDL_Rect) -> c_int;
    pub fn SDL_LowerBlitScaled(src: *SDL_Surface, srcrect: *SDL_Rect, dst: *SDL_Surface, dstrect: *SDL_Rect) -> c_int)*/
}
