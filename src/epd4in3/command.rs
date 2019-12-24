use crate::color::Color;
use crate::traits;
use arrayvec::ArrayVec;
use encoding_rs::*;

#[repr(u8)]
pub enum Command {
    Handshake = 0,
    LoadFont = 0x0e,
    LoadBmp = 0x0f,
    Clear = 0x2e,
    Update = 0x0a,
    Sleep = 0x08,
    SetRotation = 0x0D,
    SetColor = 0x10,
    SetFontSizeEn = 0x1E,
    SetFontSizeZh = 0x1F,
    Point = 0x20,
    Line = 0x22,
    Rect = 0x25,
    FillRect = 0x24,
    Circle = 0x26,
    FillCircle = 0x27,
    Tri = 0x28,
    FillTri = 0x29,
    Text = 0x30,
    Bmp = 0x70,
}

impl Command {
    fn value(self) -> u8 {
        self as u8
    }
}

impl traits::Command for Command {
    /// Returns the address of the command
    fn address(self) -> u8 {
        self as u8
    }
}

#[repr(u8)]
pub enum Rotation {
    Rotation0 = 0,
    Rotation180 = 1,
}

#[repr(u8)]
pub enum Fontsize {
    Size32 = 1,
    Size48 = 2,
    Size64 = 3,
}

#[allow(dead_code)]
pub struct Frame {
    len: u16,
    bytes: [u8; 1033],
}

impl Frame {
    pub fn get_bytes(&self) -> &[u8] {
        &self.bytes[0..self.len as usize]
    }
}

fn push_u16_to_array(value: u16, array: &mut ArrayVec<[u8; 1024]>) {
    for byte in value.to_be_bytes().iter() {
        array.push(*byte);
    }
}

fn build_frame(cmd: Command, args: &ArrayVec<[u8; 1024]>) -> Option<Frame> {
    let len: u16 = 9 + args.len() as u16;
    if len > 1033 {
        return None;
    };

    let mut bytes = [0; 1033];
    let mut pos: usize = 1;
    bytes[0] = 0xA5;

    for byte in len.to_be_bytes().iter() {
        bytes[pos] = *byte;
        pos += 1;
    }

    bytes[pos] = cmd.value();
    pos += 1;

    for byte in args.iter() {
        bytes[pos] = *byte;
        pos += 1;
    }

    let frame_end: [u8; 4] = [0xcc, 0x33, 0xc3, 0x3c];
    for byte in frame_end.iter() {
        bytes[pos] = *byte;
        pos += 1;
    }

    let mut parity: u8 = 0x00;
    for byte in bytes[0..pos].iter() {
        parity ^= *byte;
    }

    bytes[pos] = parity;

    Some(Frame { len, bytes })
}

pub fn handshake() -> Option<Frame> {
    build_frame(Command::Handshake, &ArrayVec::<[_; 1024]>::new())
}

pub fn load_font() -> Option<Frame> {
    build_frame(Command::LoadFont, &ArrayVec::<[_; 1024]>::new())
}

pub fn load_bmp() -> Option<Frame> {
    build_frame(Command::LoadBmp, &ArrayVec::<[_; 1024]>::new())
}

pub fn clear() -> Option<Frame> {
    build_frame(Command::Clear, &ArrayVec::<[_; 1024]>::new())
}

pub fn refresh() -> Option<Frame> {
    build_frame(Command::Update, &ArrayVec::<[_; 1024]>::new())
}

pub fn sleep() -> Option<Frame> {
    build_frame(Command::Sleep, &ArrayVec::<[_; 1024]>::new())
}

pub fn set_rotation(rot: Rotation) -> Option<Frame> {
    let mut array = ArrayVec::<[_; 1024]>::new();
    array.push(rot as u8);
    build_frame(Command::SetRotation, &array)
}

pub fn point(x0: u16, y0: u16) -> Option<Frame> {
    let mut array = ArrayVec::<[_; 1024]>::new();
    push_u16_to_array(x0, &mut array);
    push_u16_to_array(y0, &mut array);

    build_frame(Command::Point, &array)
}

pub fn line(x0: u16, y0: u16, x1: u16, y1: u16) -> Option<Frame> {
    let mut array = ArrayVec::<[_; 1024]>::new();
    push_u16_to_array(x0, &mut array);
    push_u16_to_array(y0, &mut array);
    push_u16_to_array(x1, &mut array);
    push_u16_to_array(y1, &mut array);

    build_frame(Command::Line, &array)
}

pub fn rect(x0: u16, y0: u16, x1: u16, y1: u16) -> Option<Frame> {
    let mut array = ArrayVec::<[_; 1024]>::new();
    push_u16_to_array(x0, &mut array);
    push_u16_to_array(y0, &mut array);
    push_u16_to_array(x1, &mut array);
    push_u16_to_array(y1, &mut array);

    build_frame(Command::Rect, &array)
}

pub fn fill_rect(x0: u16, y0: u16, x1: u16, y1: u16) -> Option<Frame> {
    let mut array = ArrayVec::<[_; 1024]>::new();
    push_u16_to_array(x0, &mut array);
    push_u16_to_array(y0, &mut array);
    push_u16_to_array(x1, &mut array);
    push_u16_to_array(y1, &mut array);

    build_frame(Command::FillRect, &array)
}

pub fn circle(x0: u16, y0: u16, r: u16) -> Option<Frame> {
    let mut array = ArrayVec::<[_; 1024]>::new();
    push_u16_to_array(x0, &mut array);
    push_u16_to_array(y0, &mut array);
    push_u16_to_array(r, &mut array);

    build_frame(Command::Circle, &array)
}

pub fn fill_circle(x0: u16, y0: u16, r: u16) -> Option<Frame> {
    let mut array = ArrayVec::<[_; 1024]>::new();
    push_u16_to_array(x0, &mut array);
    push_u16_to_array(y0, &mut array);
    push_u16_to_array(r, &mut array);

    build_frame(Command::FillCircle, &array)
}

pub fn tri(x0: u16, y0: u16, x1: u16, y1: u16, x2: u16, y2: u16) -> Option<Frame> {
    let mut array = ArrayVec::<[_; 1024]>::new();
    push_u16_to_array(x0, &mut array);
    push_u16_to_array(y0, &mut array);
    push_u16_to_array(x1, &mut array);
    push_u16_to_array(y1, &mut array);
    push_u16_to_array(x2, &mut array);
    push_u16_to_array(y2, &mut array);

    build_frame(Command::Tri, &array)
}

pub fn fill_tri(x0: u16, y0: u16, x1: u16, y1: u16, x2: u16, y2: u16) -> Option<Frame> {
    let mut array = ArrayVec::<[_; 1024]>::new();
    push_u16_to_array(x0, &mut array);
    push_u16_to_array(y0, &mut array);
    push_u16_to_array(x1, &mut array);
    push_u16_to_array(y1, &mut array);
    push_u16_to_array(x2, &mut array);
    push_u16_to_array(y2, &mut array);

    build_frame(Command::FillTri, &array)
}

pub fn text(x0: u16, y0: u16, txt: &str) -> Option<Frame> {
    let mut array = ArrayVec::<[_; 1024]>::new();
    push_u16_to_array(x0, &mut array);
    push_u16_to_array(y0, &mut array);

    let res = GBK.encode(txt);

    for byte in res.0.iter() {
        array.push(*byte);
    }
    array.push(0x00);

    build_frame(Command::Text, &array)
}

pub fn bmp(x0: u16, y0: u16, txt: &str) -> Option<Frame> {
    if txt.len() > 11 {
        return None;
    };
    if !txt.is_ascii() {
        return None;
    };

    let mut array = ArrayVec::<[_; 1024]>::new();
    push_u16_to_array(x0, &mut array);
    push_u16_to_array(y0, &mut array);

    for byte in txt.as_bytes().iter() {
        array.push(*byte);
    }
    array.push(0x00);

    build_frame(Command::Bmp, &array)
}

pub fn set_font_size_en(fontsize: Fontsize) -> Option<Frame> {
    let mut array = ArrayVec::<[_; 1024]>::new();
    array.push(fontsize as u8);
    build_frame(Command::SetFontSizeEn, &array)
}

pub fn set_font_size_zh(fontsize: Fontsize) -> Option<Frame> {
    let mut array = ArrayVec::<[_; 1024]>::new();
    array.push(fontsize as u8);
    build_frame(Command::SetFontSizeZh, &array)
}

pub fn set_color(foreground: Color, background: Color) -> Option<Frame> {
    let mut array = ArrayVec::<[_; 1024]>::new();
    array.push(foreground as u8);
    array.push(background as u8);
    build_frame(Command::SetColor, &array)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn handshake_works() {
        let frame = handshake().unwrap();
        assert_eq!(
            frame.bytes[0..(frame.len as usize)],
            [0xA5, 0x00, 0x09, 0x00, 0xCC, 0x33, 0xC3, 0x3C, 0xAC]
        );
    }

    #[test]
    fn load_font_works() {
        let frame = load_font().unwrap();
        assert_eq!(
            frame.bytes[0..(frame.len as usize)],
            [0xA5, 0x00, 0x09, 0x0e, 0xCC, 0x33, 0xC3, 0x3C, 0xA2]
        );
    }

    #[test]
    fn load_bmp_works() {
        let frame = load_bmp().unwrap();
        assert_eq!(
            frame.bytes[0..(frame.len as usize)],
            [0xA5, 0x00, 0x09, 0x0f, 0xCC, 0x33, 0xC3, 0x3C, 0xA3]
        );
    }

    #[test]
    fn clear_works() {
        let frame = clear().unwrap();
        assert_eq!(
            frame.bytes[0..(frame.len as usize)],
            [0xA5, 0x00, 0x09, 0x2e, 0xCC, 0x33, 0xC3, 0x3C, 0x82]
        );
    }

    #[test]
    fn refresh_works() {
        let frame = refresh().unwrap();
        assert_eq!(
            frame.bytes[0..(frame.len as usize)],
            [0xA5, 0x00, 0x09, 0x0a, 0xCC, 0x33, 0xC3, 0x3C, 0xa6]
        );
    }

    #[test]
    fn sleep_works() {
        let frame = sleep().unwrap();
        assert_eq!(
            frame.bytes[0..(frame.len as usize)],
            [0xA5, 0x00, 0x09, 0x08, 0xCC, 0x33, 0xC3, 0x3C, 0xa4]
        );
    }

    #[test]
    fn set_rotation_works() {
        let frame = set_rotation(Rotation::Rotation180).unwrap();
        assert_eq!(
            frame.bytes[0..(frame.len as usize)],
            [0xA5, 0x00, 0x0a, 0x0d, 0x01, 0xCC, 0x33, 0xC3, 0x3C, 0xa3]
        );
    }

    #[test]
    fn point_works() {
        let frame = point(0xa, 0xa).unwrap();
        assert_eq!(
            frame.bytes[0..(frame.len as usize)],
            [0xA5, 0x00, 0x0d, 0x20, 0x00, 0x0a, 0x00, 0x0a, 0xCC, 0x33, 0xC3, 0x3C, 0x88]
        );
    }

    #[test]
    fn line_works() {
        let frame = line(0xa, 0xa, 0xff, 0xff).unwrap();
        assert_eq!(
            frame.bytes[0..(frame.len as usize)],
            [
                0xA5, 0x00, 0x11, 0x22, 0x00, 0x0a, 0x00, 0x0a, 0x00, 0xff, 0x00, 0xff, 0xCC, 0x33,
                0xC3, 0x3C, 0x96
            ]
        );
    }

    #[test]
    fn rect_works() {
        let frame = rect(0xa, 0xa, 0xff, 0xff).unwrap();
        assert_eq!(
            frame.bytes[0..(frame.len as usize)],
            [
                0xA5, 0x00, 0x11, 0x25, 0x00, 0x0a, 0x00, 0x0a, 0x00, 0xff, 0x00, 0xff, 0xCC, 0x33,
                0xC3, 0x3C, 0x91
            ]
        );
    }

    #[test]
    fn fill_rect_works() {
        let frame = fill_rect(0xa, 0xa, 0xff, 0xff).unwrap();
        assert_eq!(
            frame.bytes[0..(frame.len as usize)],
            [
                0xA5, 0x00, 0x11, 0x24, 0x00, 0x0a, 0x00, 0x0a, 0x00, 0xff, 0x00, 0xff, 0xCC, 0x33,
                0xC3, 0x3C, 0x90
            ]
        );
    }

    #[test]
    fn circle_works() {
        let frame = circle(0xff, 0xff, 0x80).unwrap();
        assert_eq!(
            frame.bytes[0..(frame.len as usize)],
            [
                0xA5, 0x00, 0x0f, 0x26, 0x00, 0xff, 0x00, 0xff, 0x00, 0x80, 0xCC, 0x33, 0xC3, 0x3C,
                0x0c
            ]
        );
    }

    #[test]
    fn fill_circle_works() {
        let frame = fill_circle(0xff, 0xff, 0x80).unwrap();
        assert_eq!(
            frame.bytes[0..(frame.len as usize)],
            [
                0xA5, 0x00, 0x0f, 0x27, 0x00, 0xff, 0x00, 0xff, 0x00, 0x80, 0xCC, 0x33, 0xC3, 0x3C,
                0x0d
            ]
        );
    }

    #[test]
    fn tri_works() {
        let frame = tri(0x0a, 0x0a, 0x20, 0x80, 0x80, 0xff).unwrap();
        assert_eq!(
            frame.bytes[0..(frame.len as usize)],
            [
                0xA5, 0x00, 0x15, 0x28, 0x00, 0x0a, 0x00, 0x0a, 0x00, 0x20, 0x00, 0x80, 0x00, 0x80,
                0x00, 0xff, 0xCC, 0x33, 0xC3, 0x3C, 0x47
            ]
        );
    }

    #[test]
    fn fill_tri_works() {
        let frame = fill_tri(0x0a, 0x0a, 0x20, 0x80, 0x80, 0xff).unwrap();
        assert_eq!(
            frame.bytes[0..(frame.len as usize)],
            [
                0xA5, 0x00, 0x15, 0x29, 0x00, 0x0a, 0x00, 0x0a, 0x00, 0x20, 0x00, 0x80, 0x00, 0x80,
                0x00, 0xff, 0xCC, 0x33, 0xC3, 0x3C, 0x46
            ]
        );
    }

    #[test]
    fn text_works() {
        let txt = "你好World";
        let frame = text(0x0a, 0x0a, &txt).unwrap();
        assert_eq!(
            frame.bytes[0..(frame.len as usize)],
            [
                0xA5, 0x00, 0x17, 0x30, 0x00, 0x0a, 0x00, 0x0a, 0xC4, 0xE3, 0xBA, 0xC3, 0x57, 0x6F,
                0x72, 0x6C, 0x64, 0x00, 0xCC, 0x33, 0xC3, 0x3C, 0x9e
            ]
        );
    }

    #[test]
    fn bmp_works() {
        let txt = "PIC7.BMP";
        let frame = bmp(0x00, 0x00, &txt).unwrap();
        assert_eq!(
            frame.bytes[0..(frame.len as usize)],
            [
                0xA5, 0x00, 0x16, 0x70, 0x00, 0x00, 0x00, 0x00, 0x50, 0x49, 0x43, 0x37, 0x2e, 0x42,
                0x4d, 0x50, 0x00, 0xCC, 0x33, 0xC3, 0x3C, 0xdf
            ]
        );
    }

    #[test]
    fn bmp_non_ascii_works() {
        let txt = "你C7.BMP";
        let res = bmp(0x00, 0x00, &txt);
        assert!(res.is_none());
    }

    #[test]
    fn bmp_length_incorrect_works() {
        let txt = "TOOLONGNAME.BMP";
        let res = bmp(0x00, 0x00, &txt);
        assert!(res.is_none());
    }

    #[test]
    fn set_font_size_en_works() {
        let frame = set_font_size_en(Fontsize::Size64).unwrap();
        assert_eq!(
            frame.bytes[0..(frame.len as usize)],
            [0xA5, 0x00, 0x0A, 0x1E, 0x03, 0xCC, 0x33, 0xC3, 0x3C, 0xB2]
        );
    }

    #[test]
    fn set_font_size_zh_works() {
        let frame = set_font_size_zh(Fontsize::Size32).unwrap();
        assert_eq!(
            frame.bytes[0..(frame.len as usize)],
            [0xA5, 0x00, 0x0A, 0x1F, 0x01, 0xCC, 0x33, 0xC3, 0x3C, 0xB1]
        );
    }

    #[test]
    fn set_color_works() {
        let frame = set_color(Color::Black, Color::White).unwrap();
        assert_eq!(
            frame.bytes[0..(frame.len as usize)],
            [0xA5, 0x00, 0x0B, 0x10, 0x00, 0x03, 0xCC, 0x33, 0xC3, 0x3C, 0xBD]
        );
    }
}
