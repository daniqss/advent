pub type DayResult = Option<String>;
pub trait Day {
    fn puzzle_1(&self) -> DayResult;
    fn puzzle_2(&self) -> DayResult;
}
