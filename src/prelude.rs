pub type DayResult = Option<String>;
pub type Day = (fn() -> DayResult, fn() -> DayResult);
