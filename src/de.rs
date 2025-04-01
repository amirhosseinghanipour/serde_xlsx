use crate::error::Error;
use rust_xlsxwriter::Workbook;
use serde::{Serialize, ser};
use std::io::{Seek, Write};

pub fn to_xlsx<T: Serialize, W: Write + Seek + Send>(value: &T, writer: W) -> Result<(), Error> {
    let mut serializer = XlsxSerializer::new(writer)?;
    value.serialize(&mut serializer)?;
    serializer.finish()?;
    Ok(())
}

pub struct XlsxSerializer<W: Write + Seek + Send> {
    workbook: Workbook,
    writer: W,
    row: u32,
    col: u16,
}

impl<W: Write + Seek + Send> XlsxSerializer<W> {
    pub fn new(writer: W) -> Result<Self, Error> {
        let mut workbook = Workbook::new();
        let _ = workbook.add_worksheet();
        Ok(Self {
            workbook,
            writer,
            row: 0,
            col: 0,
        })
    }

    pub fn finish(mut self) -> Result<(), Error> {
        self.workbook
            .save_to_writer(self.writer)
            .map_err(|e| Error::Custom(e.to_string()))?;
        Ok(())
    }
}

impl<'a, W: Write + Seek + Send> ser::Serializer for &'a mut XlsxSerializer<W> {
    type Ok = ();
    type Error = Error;
    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        let worksheet = self.workbook.worksheets_mut().get_mut(0).unwrap();
        worksheet.write_boolean(self.row, self.col, v)?;
        self.col += 1;
        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(v as i64)
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(v as i64)
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(v as i64)
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        let worksheet = self.workbook.worksheets_mut().get_mut(0).unwrap();
        worksheet.write_number(self.row, self.col, v as f64)?;
        self.col += 1;
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.serialize_u64(v as u64)
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.serialize_u64(v as u64)
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.serialize_u64(v as u64)
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        let worksheet = self.workbook.worksheets_mut().get_mut(0).unwrap();
        worksheet.write_number(self.row, self.col, v as f64)?;
        self.col += 1;
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.serialize_f64(v as f64)
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        let worksheet = self.workbook.worksheets_mut().get_mut(0).unwrap();
        worksheet.write_number(self.row, self.col, v)?;
        self.col += 1;
        Ok(())
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(&v.to_string())
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        let worksheet = self.workbook.worksheets_mut().get_mut(0).unwrap();
        worksheet.write_string(self.row, self.col, v)?;
        self.col += 1;
        Ok(())
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Err(Error::Custom(
            "Bytes serialization not supported".to_string(),
        ))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.serialize_unit()
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(variant)
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        self.serialize_str(variant)?;
        value.serialize(self)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(self)
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Ok(self)
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Ok(self)
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Ok(self)
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(self)
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Ok(self)
    }
}

impl<'a, W: Write + Seek + Send> ser::SerializeSeq for &'a mut XlsxSerializer<W> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.row += 1;
        self.col = 0;
        Ok(())
    }
}

impl<'a, W: Write + Seek + Send> ser::SerializeTuple for &'a mut XlsxSerializer<W> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.row += 1;
        self.col = 0;
        Ok(())
    }
}

impl<'a, W: Write + Seek + Send> ser::SerializeTupleStruct for &'a mut XlsxSerializer<W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.row += 1;
        self.col = 0;
        Ok(())
    }
}

impl<'a, W: Write + Seek + Send> ser::SerializeTupleVariant for &'a mut XlsxSerializer<W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.row += 1;
        self.col = 0;
        Ok(())
    }
}

impl<'a, W: Write + Seek + Send> ser::SerializeMap for &'a mut XlsxSerializer<W> {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        key.serialize(&mut **self)?;
        Ok(())
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.row += 1;
        self.col = 0;
        Ok(())
    }
}

impl<'a, W: Write + Seek + Send> ser::SerializeStruct for &'a mut XlsxSerializer<W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        _key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.row += 1;
        self.col = 0;
        Ok(())
    }
}

impl<'a, W: Write + Seek + Send> ser::SerializeStructVariant for &'a mut XlsxSerializer<W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        _key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.row += 1;
        self.col = 0;
        Ok(())
    }
}
