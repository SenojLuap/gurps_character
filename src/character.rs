#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Character {
    pub name: String,

    pub strength: u16,
    pub dexterity: u16,
    pub intelligence: u16,
    pub health: u16,

    pub will: u16,
    pub perception: u16,

    pub health_points_maximum: u16,
    pub health_points_current: u16,
    pub fatigue_points_maximum: u16,
    pub fatigue_points_current: u16,

    pub advantages: Vec<Advantage>,
    pub disadvantages: Vec<Advantage>,
}

impl Writable for Character {
    fn write<W: Write>(&self, write: &mut W) -> Result<()> {
        write.write_ext(&self.name)?;
        write.write_ext(&self.strength)?;
        write.write_ext(&self.dexterity)?;
        write.write_ext(&self.intelligence)?;
        write.write_ext(&self.health)?;
        write.write_ext(&self.will)?;
        write.write_ext(&self.perception)?;
        write.write_ext(&self.health_points_maximum)?;
        write.write_ext(&self.health_points_current)?;
        write.write_ext(&self.fatigue_points_maximum)?;
        write.write_ext(&self.fatigue_points_current)?;
        write.write_ext(&self.advantages)?;
        write.write_ext(&self.disadvantages)
    }
}

impl Readable for Character {
    fn read<R: Read>(read: &mut R) -> Result<Self> {
        let name = read.read_ext()?;
        let strength = read.read_ext()?;
        let dexterity = read.read_ext()?;
        let intelligence = read.read_ext()?;
        let health = read.read_ext()?;
        let will = read.read_ext()?;
        let perception = read.read_ext()?;
        let health_points_maximum = read.read_ext()?;
        let health_points_current = read.read_ext()?;
        let fatigue_points_maximum = read.read_ext()?;
        let fatigue_points_current = read.read_ext()?;
        let advantages = read.read_ext()?;
        let disadvantages = read.read_ext()?;
        Ok(Character{name, strength, dexterity, intelligence, health, will, perception, health_points_maximum, health_points_current, fatigue_points_maximum, fatigue_points_current, advantages, disadvantages})
    }
}

use std::io::{Read, Result, Write};

use read_write_ext::{ReadExt, Readable, Writable, WriteExt};
use serde::{Serialize, Deserialize};

use crate::Advantage;

