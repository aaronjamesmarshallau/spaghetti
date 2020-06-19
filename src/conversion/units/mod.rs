use diesel::backend::Backend;
use diesel::deserialize::FromSql;
use diesel::serialize::Output;
use diesel::sql_types::SmallInt;
use diesel::types::ToSql;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, AsExpression, FromSqlRow)]
#[sql_type = "SmallInt"]
pub enum UnitOfMeasurement {
    None,
    // Units of Mass (Metric)
    Milligrams,
    Grams,
    Kilograms,

    // Units of Mass (Imperials)
    Ounces,
    Pounds,

    // Units of Volume (general)
    Teaspoons,
    Tablespoons,
    Cups,

    // Units of Volume (liquid, metric)
    Millilitres,
    Litres,

    // Units of Volume (liquid, imperial)
    FluidOunces,

    // Miscellaneous
    Pinch,
    Dash,
}

impl<DB: Backend> ToSql<SmallInt, DB> for UnitOfMeasurement
where
    i16: ToSql<SmallInt, DB>,
{
    fn to_sql<W>(&self, out: &mut Output<W, DB>) -> diesel::serialize::Result
    where
        W: std::io::Write,
    {
        i16::from(self).to_sql(out)
    }
}

impl<DB: Backend> FromSql<SmallInt, DB> for UnitOfMeasurement
where
    i16: FromSql<SmallInt, DB>,
{
    fn from_sql(bytes: Option<&DB::RawValue>) -> diesel::deserialize::Result<Self> {
        let small_int = i16::from_sql(bytes)?;

        Ok(UnitOfMeasurement::from(small_int))
    }
}

impl From<i16> for UnitOfMeasurement {
    fn from(num: i16) -> UnitOfMeasurement {
        match num {
            // Units of Mass (Metric)
            1 => UnitOfMeasurement::Milligrams,
            2 => UnitOfMeasurement::Grams,
            3 => UnitOfMeasurement::Kilograms,

            // Units of Mass (Imperials)
            4 => UnitOfMeasurement::Ounces,
            5 => UnitOfMeasurement::Pounds,

            // Units of Volume (general)
            6 => UnitOfMeasurement::Teaspoons,
            7 => UnitOfMeasurement::Tablespoons,
            8 => UnitOfMeasurement::Cups,

            // Units of Volume (liquid, metric)
            9 => UnitOfMeasurement::Millilitres,
            10 => UnitOfMeasurement::Litres,

            // Units of Volume (liquid, imperial)
            11 => UnitOfMeasurement::FluidOunces,

            // Miscellaneous
            12 => UnitOfMeasurement::Pinch,
            13 => UnitOfMeasurement::Dash,

            // None
            _ => UnitOfMeasurement::None,
        }
    }
}

impl From<&UnitOfMeasurement> for i16 {
    fn from(unit: &UnitOfMeasurement) -> i16 {
        match unit {
            // Units of Mass (Metric)
            UnitOfMeasurement::Milligrams => 1,
            UnitOfMeasurement::Grams => 2,
            UnitOfMeasurement::Kilograms => 3,

            // Units of Mass (Imperials)
            UnitOfMeasurement::Ounces => 4,
            UnitOfMeasurement::Pounds => 5,

            // Units of Volume (general)
            UnitOfMeasurement::Teaspoons => 6,
            UnitOfMeasurement::Tablespoons => 7,
            UnitOfMeasurement::Cups => 8,

            // Units of Volume (liquid, metric)
            UnitOfMeasurement::Millilitres => 9,
            UnitOfMeasurement::Litres => 10,

            // Units of Volume (liquid, imperial)
            UnitOfMeasurement::FluidOunces => 11,

            // Miscellaneous
            UnitOfMeasurement::Pinch => 12,
            UnitOfMeasurement::Dash => 13,

            // None
            _ => 0,
        }
    }
}

impl From<UnitOfMeasurement> for i16 {
    fn from(unit: UnitOfMeasurement) -> i16 {
        *(&unit.into())
    }
}
