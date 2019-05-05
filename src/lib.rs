#![feature(test)]

#[macro_use]
extern crate serde_derive;
extern crate test;

use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub enum Frame<'a> {
    #[serde(borrow)]
    Yuv(YuvFrame<'a>),
    #[serde(borrow)]
    Rgb(RgbFrame<'a>),
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct YuvFrameInfo {
    pub width: u32,
    pub height: u32,
    pub y_stride: u32,
    pub u_stride: u32,
    pub v_stride: u32,
}

#[derive(Serialize, Deserialize)]
pub struct YuvFrame<'a> {
    pub info: YuvFrameInfo,
    #[serde(with = "serde_bytes")]
    pub y_pixels: &'a [u8],
    #[serde(with = "serde_bytes")]
    pub u_pixels: &'a [u8],
    #[serde(with = "serde_bytes")]
    pub v_pixels: &'a [u8],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct RgbFrameInfo {
    pub width: u32,
    pub height: u32,
    pub stride: u32,
}

#[derive(Serialize, Deserialize)]
pub struct RgbFrame<'a> {
    pub info: RgbFrameInfo,
    #[serde(with = "serde_bytes")]
    pub pixels: &'a [u8],
}

#[derive(Serialize, Deserialize)]
pub enum Message<'a> {
    CreateConnection(Uuid),
    DestroyConnection(Uuid),
    Frame {
        connection: Uuid,
        #[serde(borrow)]
        frame: Frame<'a>,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    const WIDTH: usize = 3840;
    const HEIGHT: usize = 2160;
    const Y_STRIDE: usize = WIDTH;
    const U_STRIDE: usize = WIDTH / 2;
    const V_STRIDE: usize = WIDTH / 2;

    #[bench]
    fn bench_serialize_into(b: &mut Bencher) {
        let y_pixels = vec![0_u8; Y_STRIDE * HEIGHT];
        let u_pixels = vec![0_u8; U_STRIDE * HEIGHT / 2];
        let v_pixels = vec![0_u8; V_STRIDE * HEIGHT / 2];

        let message = Message::Frame {
            connection: Uuid::new_v4(),
            frame: Frame::Yuv(YuvFrame {
                info: YuvFrameInfo {
                    width: WIDTH as u32,
                    height: HEIGHT as u32,
                    y_stride: Y_STRIDE as u32,
                    u_stride: U_STRIDE as u32,
                    v_stride: V_STRIDE as u32,
                },
                y_pixels: &y_pixels,
                u_pixels: &u_pixels,
                v_pixels: &v_pixels,
            }),
        };

        let size = bincode::serialized_size(&message).unwrap() as usize;
        let mut buffer = vec![0_u8; size];

        b.iter(move || bincode::serialize_into(&mut buffer as &mut [u8], &message).unwrap());
    }

    #[bench]
    fn bench_deserialize(b: &mut Bencher) {
        let y_pixels = vec![0_u8; Y_STRIDE * HEIGHT];
        let u_pixels = vec![0_u8; U_STRIDE * HEIGHT / 2];
        let v_pixels = vec![0_u8; V_STRIDE * HEIGHT / 2];

        let message = Message::Frame {
            connection: Uuid::new_v4(),
            frame: Frame::Yuv(YuvFrame {
                info: YuvFrameInfo {
                    width: WIDTH as u32,
                    height: HEIGHT as u32,
                    y_stride: Y_STRIDE as u32,
                    u_stride: U_STRIDE as u32,
                    v_stride: V_STRIDE as u32,
                },
                y_pixels: &y_pixels,
                u_pixels: &u_pixels,
                v_pixels: &v_pixels,
            }),
        };

        let size = bincode::serialized_size(&message).unwrap() as usize;
        let mut buffer = vec![0_u8; size];
        bincode::serialize_into(&mut buffer as &mut [u8], &message).unwrap();

        b.iter(move || {
            drop(test::black_box(
                bincode::deserialize::<Message>(&buffer).unwrap(),
            ))
        });
    }

    #[bench]
    fn bench_copy_message(b: &mut Bencher) {
        let y_pixels = vec![0_u8; Y_STRIDE * HEIGHT];
        let u_pixels = vec![0_u8; U_STRIDE * HEIGHT / 2];
        let v_pixels = vec![0_u8; V_STRIDE * HEIGHT / 2];

        let message = Message::Frame {
            connection: Uuid::new_v4(),
            frame: Frame::Yuv(YuvFrame {
                info: YuvFrameInfo {
                    width: WIDTH as u32,
                    height: HEIGHT as u32,
                    y_stride: Y_STRIDE as u32,
                    u_stride: U_STRIDE as u32,
                    v_stride: V_STRIDE as u32,
                },
                y_pixels: &y_pixels,
                u_pixels: &u_pixels,
                v_pixels: &v_pixels,
            }),
        };

        let size = bincode::serialized_size(&message).unwrap() as usize;
        let mut buffer = vec![0_u8; size];
        bincode::serialize_into(&mut buffer as &mut [u8], &message).unwrap();
        let mut dst = vec![0_u8; size];

        b.iter(move || dst.copy_from_slice(&buffer));
    }
}
