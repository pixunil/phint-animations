use std::fmt;
use serde::de::{self, Error, Unexpected, Deserialize, Deserializer, Visitor, MapAccess};

use graphics::{Style, Point};

mod grammar {
    include!(concat!(env!("OUT_DIR"), "/deserialize.rs"));
}

struct LineWidthVisitor;

impl<'de> Visitor<'de> for LineWidthVisitor {
    type Value = f64;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a float, `thin` or `thick`")
    }

    fn visit_i64<E>(self, value: i64) -> Result<f64, E>
        where E: de::Error
    {
        Ok(value as f64)
    }

    fn visit_u64<E>(self, value: u64) -> Result<f64, E>
        where E: de::Error
    {
        Ok(value as f64)
    }

    fn visit_f64<E>(self, value: f64) -> Result<f64, E>
        where E: de::Error
    {
        Ok(value)
    }

    fn visit_str<E>(self, value: &str) -> Result<f64, E>
        where E: de::Error
    {
        match value {
            "thin" => Ok(0.05),
            "thick" => Ok(0.1),
            invalid => Err(E::invalid_value(Unexpected::Str(invalid), &self))
        }
    }
}

pub fn line_width<'de, D>(deserializer: D) -> Result<f64, D::Error>
    where D: Deserializer<'de>
{
    deserializer.deserialize_f64(LineWidthVisitor)
}

struct AngleVisitor;

impl<'de> Visitor<'de> for AngleVisitor {
    type Value = f64;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an angle in radians or degrees")
    }

    fn visit_i64<E>(self, value: i64) -> Result<f64, E>
        where E: de::Error
    {
        // interpret plain integers as radians
        Ok(value as f64)
    }

    fn visit_u64<E>(self, value: u64) -> Result<f64, E>
        where E: de::Error
    {
        // interpret plain integers as radians
        Ok(value as f64)
    }

    fn visit_f64<E>(self, value: f64) -> Result<f64, E>
        where E: de::Error
    {
        // interpret plain floats as radians
        Ok(value)
    }

    fn visit_str<E>(self, value: &str) -> Result<f64, E>
        where E: de::Error
    {
        // use custom grammar for strings
        grammar::angle(value).map_err(de::Error::custom)
    }
}

pub fn angle<'de, D>(deserializer: D) -> Result<f64, D::Error>
    where D: Deserializer<'de>
{
    deserializer.deserialize_f64(AngleVisitor)
}

struct StyleVisitor;

impl<'de> Visitor<'de> for StyleVisitor {
    type Value = Style;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("one of `stroke` or `fill`")
    }

    fn visit_str<E>(self, value: &str) -> Result<Style, E>
        where E: de::Error
    {
        match value {
            "stroke" => Ok(Style::Stroke),
            "fill" => Ok(Style::Fill),
            invalid => Err(E::invalid_value(Unexpected::Str(invalid), &self))
        }
    }
}

impl<'de> Deserialize<'de> for Style {
    fn deserialize<D>(deserializer: D) -> Result<Style, D::Error>
        where D: Deserializer<'de>
    {
        deserializer.deserialize_str(StyleVisitor)
    }
}

static POINT_FIELDS: &[&str] = &["x", "y"];

struct PointVisitor;

impl<'de> Visitor<'de> for PointVisitor {
    type Value = Point;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a point")
    }

    fn visit_str<E>(self, value: &str) -> Result<Point, E>
        where E: de::Error
    {
        grammar::point(value).map_err(E::custom)
    }

    fn visit_map<M>(self, mut access: M) -> Result<Point, M::Error>
        where M: MapAccess<'de>
    {
        let mut x = None;
        let mut y = None;

        while let Some((key, value)) = access.next_entry::<String, f64>()? {
            match key.as_ref() {
                "x" => x = Some(value),
                "y" => y = Some(value),
                _ => return Err(M::Error::unknown_field(&key, POINT_FIELDS))
            }
        }

        let x = match x {
            Some(x) => x,
            None => return Err(M::Error::missing_field("x"))
        };

        let y = match y {
            Some(y) => y,
            None => return Err(M::Error::missing_field("y"))
        };

        Ok(Point::new(x, y))
    }
}

impl<'de> Deserialize<'de> for Point {
    fn deserialize<D>(deserializer: D) -> Result<Point, D::Error>
        where D: Deserializer<'de>
    {
        deserializer.deserialize_struct("Point", POINT_FIELDS, PointVisitor)
    }
}
