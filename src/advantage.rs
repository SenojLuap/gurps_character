/// A character advantage.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Advantage {
    pub points : u32,
    pub levels : u32,
    pub qualifier: Option<String>,

    pub physical: bool,
    pub mental: bool,
    pub social: bool,
    pub exotic: bool,
    pub supernatural: bool,

    pub short_description: String,
    pub full_description: Option<String>,
    pub book_ref: Option<String>,

    pub modifiers: Vec<AdvantageModifier>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvantageModifier {
    pub short_description: String,
    pub full_description: Option<String>,
}

impl Writable for Advantage {
    fn write<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_ext(&self.points)?;
        writer.write_ext(&self.levels)?;
        writer.write_ext(&self.qualifier)?;
        writer.write_ext(&self.physical)?;
        writer.write_ext(&self.mental)?;
        writer.write_ext(&self.social)?;
        writer.write_ext(&self.exotic)?;
        writer.write_ext(&self.supernatural)?;
        writer.write_ext(&self.short_description)?;
        writer.write_ext(&self.full_description)?;
        writer.write_ext(&self.book_ref)?;
        writer.write_ext(&self.modifiers)
    }
}

impl Readable for Advantage {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let points = reader.read_ext()?;
        let levels = reader.read_ext()?;
        let qualifier = reader.read_ext()?;
        let physical = reader.read_ext()?;
        let mental = reader.read_ext()?;
        let social = reader.read_ext()?;
        let exotic = reader.read_ext()?;
        let supernatural = reader.read_ext()?;
        let short_description = reader.read_ext()?;
        let full_description = reader.read_ext()?;
        let book_ref = reader.read_ext()?;
        let modifiers = reader.read_ext()?;
        Ok(Self{points, levels, qualifier, physical, mental, social, exotic, supernatural, short_description, full_description, book_ref, modifiers})
    }
}

impl Writable for AdvantageModifier {
    fn write<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_ext(&self.short_description)?;
        writer.write_ext(&self.full_description)
    }
}

impl Readable for AdvantageModifier {
    fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let short_description = reader.read_ext()?;
        let full_description = reader.read_ext()?;
        Ok(AdvantageModifier{short_description, full_description})
    }
}

use std::io::{Read, Result, Write};
use read_write_ext::{Readable, ReadExt, Writable, WriteExt};
use serde::{Serialize, Deserialize};
