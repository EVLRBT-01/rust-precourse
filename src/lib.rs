#[cfg(test)]
mod tests {
    use rand::seq::SliceRandom;
    use std::fmt::{Display, Formatter};

    #[test]
    fn exercise_1() {
        let result: u8 = (1..=10).sum();
        println!("{result}");
        assert_eq!(55, result);
    }

    #[test]
    fn exercise_2() {
        fn sum(input: &[i32]) -> i32 {
            input.iter().sum()
        }

        let values = [1, 2, 3, 4, 5];

        let result = sum(&values);
        println!("{result}");
        assert_eq!(15, result);

        let result = sum(&values[..3]);
        println!("{result}");
        assert_eq!(6, result);
    }

    #[allow(dead_code)]
    enum DaysOfWeek {
        Sunday,
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
    }

    impl Display for DaysOfWeek {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            f.write_str(match self {
                DaysOfWeek::Sunday => "Sunday",
                DaysOfWeek::Monday => "Monday",
                DaysOfWeek::Tuesday => "Tuesday",
                DaysOfWeek::Wednesday => "Wednesday",
                DaysOfWeek::Thursday => "Thursday",
                DaysOfWeek::Friday => "Friday",
                DaysOfWeek::Saturday => "Saturday",
            })
        }
    }

    const DAYS_OF_WEEK: [DaysOfWeek; 7] = [
        DaysOfWeek::Sunday,
        DaysOfWeek::Monday,
        DaysOfWeek::Tuesday,
        DaysOfWeek::Wednesday,
        DaysOfWeek::Thursday,
        DaysOfWeek::Friday,
        DaysOfWeek::Saturday,
    ];

    #[test]
    fn exercise_3() {
        let day = DaysOfWeek::Monday;
        println!("{day}");
    }

    #[test]
    fn exercise_4() {
        let day_of_week = DAYS_OF_WEEK.choose(&mut rand::thread_rng()).unwrap();
        let result = match day_of_week {
            DaysOfWeek::Saturday | DaysOfWeek::Sunday => "weekend",
            _ => "working",
        };
        println!("{day_of_week} is {result}");
    }

    #[test]
    fn exercise_5() {
        fn fibonacci(n: usize) -> Vec<usize> {
            let mut sequence = vec![];
            for i in 0..n {
                if i <= 1 {
                    sequence.push(i)
                } else {
                    let x = sequence.len();
                    sequence.push(sequence[x - 1] + sequence[x - 2]);
                }
            }
            sequence
        }

        println!("{:?}", fibonacci(0));
        println!("{:?}", fibonacci(1));
        println!("{:?}", fibonacci(2));
        println!("{:?}", fibonacci(10));
    }

    #[test]
    fn exercise_6() {}

    // #[test]
    // fn iterator_question() {
    //     let items  = [1,2,3];
    //     let x = items
    //         .iter()
    //         .map(|x| x)
    //         .collect();
    //
    //     let x = items
    //         .into_iter()
    //         .map(|x| x)
    //         .collect();
    // }
}
