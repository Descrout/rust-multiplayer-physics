// Automatically generated rust module for 'proto-all.proto' file

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(unknown_lints)]
#![allow(clippy::all)]
#![cfg_attr(rustfmt, rustfmt_skip)]


use quick_protobuf::{MessageRead, MessageWrite, BytesReader, Writer, WriterBackend, Result};
use quick_protobuf::sizeofs::*;
use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Entity {
    pub id: u32,
    pub x: f32,
    pub y: f32,
    pub pressed: bool,
    pub color: String,
}

impl<'a> MessageRead<'a> for Entity {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.id = r.read_uint32(bytes)?,
                Ok(21) => msg.x = r.read_float(bytes)?,
                Ok(29) => msg.y = r.read_float(bytes)?,
                Ok(32) => msg.pressed = r.read_bool(bytes)?,
                Ok(42) => msg.color = r.read_string(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Entity {
    fn get_size(&self) -> usize {
        0
        + if self.id == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.id) as u64) }
        + if self.x == 0f32 { 0 } else { 1 + 4 }
        + if self.y == 0f32 { 0 } else { 1 + 4 }
        + if self.pressed == false { 0 } else { 1 + sizeof_varint(*(&self.pressed) as u64) }
        + if self.color == String::default() { 0 } else { 1 + sizeof_len((&self.color).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != 0u32 { w.write_with_tag(8, |w| w.write_uint32(*&self.id))?; }
        if self.x != 0f32 { w.write_with_tag(21, |w| w.write_float(*&self.x))?; }
        if self.y != 0f32 { w.write_with_tag(29, |w| w.write_float(*&self.y))?; }
        if self.pressed != false { w.write_with_tag(32, |w| w.write_bool(*&self.pressed))?; }
        if self.color != String::default() { w.write_with_tag(42, |w| w.write_string(&**&self.color))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Body {
    pub id: u32,
    pub color: String,
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub rotation: f32,
}

impl<'a> MessageRead<'a> for Body {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.id = r.read_uint32(bytes)?,
                Ok(18) => msg.color = r.read_string(bytes)?.to_owned(),
                Ok(29) => msg.x = r.read_float(bytes)?,
                Ok(37) => msg.y = r.read_float(bytes)?,
                Ok(45) => msg.w = r.read_float(bytes)?,
                Ok(53) => msg.h = r.read_float(bytes)?,
                Ok(61) => msg.rotation = r.read_float(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Body {
    fn get_size(&self) -> usize {
        0
        + if self.id == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.id) as u64) }
        + if self.color == String::default() { 0 } else { 1 + sizeof_len((&self.color).len()) }
        + if self.x == 0f32 { 0 } else { 1 + 4 }
        + if self.y == 0f32 { 0 } else { 1 + 4 }
        + if self.w == 0f32 { 0 } else { 1 + 4 }
        + if self.h == 0f32 { 0 } else { 1 + 4 }
        + if self.rotation == 0f32 { 0 } else { 1 + 4 }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != 0u32 { w.write_with_tag(8, |w| w.write_uint32(*&self.id))?; }
        if self.color != String::default() { w.write_with_tag(18, |w| w.write_string(&**&self.color))?; }
        if self.x != 0f32 { w.write_with_tag(29, |w| w.write_float(*&self.x))?; }
        if self.y != 0f32 { w.write_with_tag(37, |w| w.write_float(*&self.y))?; }
        if self.w != 0f32 { w.write_with_tag(45, |w| w.write_float(*&self.w))?; }
        if self.h != 0f32 { w.write_with_tag(53, |w| w.write_float(*&self.h))?; }
        if self.rotation != 0f32 { w.write_with_tag(61, |w| w.write_float(*&self.rotation))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct State {
    pub entities: Vec<Entity>,
    pub bodies: Vec<Body>,
}

impl<'a> MessageRead<'a> for State {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.entities.push(r.read_message::<Entity>(bytes)?),
                Ok(18) => msg.bodies.push(r.read_message::<Body>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for State {
    fn get_size(&self) -> usize {
        0
        + self.entities.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.bodies.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.entities { w.write_with_tag(10, |w| w.write_message(s))?; }
        for s in &self.bodies { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GameInput {
    pub x: f32,
    pub y: f32,
    pub pressed: bool,
}

impl<'a> MessageRead<'a> for GameInput {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(13) => msg.x = r.read_float(bytes)?,
                Ok(21) => msg.y = r.read_float(bytes)?,
                Ok(24) => msg.pressed = r.read_bool(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for GameInput {
    fn get_size(&self) -> usize {
        0
        + if self.x == 0f32 { 0 } else { 1 + 4 }
        + if self.y == 0f32 { 0 } else { 1 + 4 }
        + if self.pressed == false { 0 } else { 1 + sizeof_varint(*(&self.pressed) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.x != 0f32 { w.write_with_tag(13, |w| w.write_float(*&self.x))?; }
        if self.y != 0f32 { w.write_with_tag(21, |w| w.write_float(*&self.y))?; }
        if self.pressed != false { w.write_with_tag(24, |w| w.write_bool(*&self.pressed))?; }
        Ok(())
    }
}

