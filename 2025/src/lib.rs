pub mod utils {
    use std::fmt::Display;
    use std::time::Instant;

    pub enum Day {
        Day1 = 1,
        Day2,
        Day3,
        Day4,
        Day5,
        Day6,
        Day7,
        Day8,
        Day9,
        Day10,
        Day11,
        Day12,
    }

    pub struct ExecuteDetails {
        pub day: Day,
        pub test: String,
        pub input: String,
    }

    fn execute<T: Display>(input: &str, runner: fn(input: &str) -> T) {
        let start = Instant::now(); // Record the starting time
        let result = runner(input);
        let duration = start.elapsed();
        println!("Result: {result} elapsed: {:?}", duration);
    }

    pub fn execute_day<T: Display, P: Display>(
        day: Day,
        test: &str,
        input: &str,
        first: fn(input: &str) -> T,
        second: fn(input: &str) -> P,
    ) {
        let day_num = day as i8;
        println!("------ Day: {day_num} ------");
        print!("Part 1 Test ");
        execute(test, first);
        print!("Part 1 Actual ");
        execute(input, first);
        print!("Part 2 Test ");
        execute(test, second);
        print!("Part 2 Actual ");
        execute(input, second);
        println!();
    }
}
