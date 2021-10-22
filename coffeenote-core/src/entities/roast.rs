use super::{bean::Bean, CoffeeNoteEntity, Datetime};
use uuid::Uuid;

// degree of roast lineup. ref: https://goodsensecoffee.com/blogs/coffee_info/degree-of-roast
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum Degree {
    LightCity,
    HalfCity,
    Cinnamon,
    NewEngland,
    Regular,
    American,
    City,
    Breakfast,
    HighCity,
    FullCity,
    HighFullCity,
    AfterDinner,
    Vienna,
    French,
    Italian,
    Espresso,
    Continental,
    NewOrleans,
    Spanish,
}

pub struct RoastId(Uuid);
pub struct RoastedAt(Datetime);
pub struct FirstCrackedSeconds(usize);
pub struct SecondCrackedSeconds(usize);
pub struct Note(String);

pub struct Roast {
    pub id: RoastId,
    pub bean: Bean,
    pub roasted_at: RoastedAt,
    pub first_cracked_seconds: FirstCrackedSeconds,
    pub second_cracked_seconds: SecondCrackedSeconds,
    pub degree: Degree,
    pub note: Note,
}
