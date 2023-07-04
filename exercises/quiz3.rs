// quiz3.rs
// This quiz tests:
// - Generics
// - Traits
// An imaginary magical school has a new report card generation system written in Rust!
// Currently the system only supports creating report cards where the student's grade
// is represented numerically (e.g. 1.0 -> 5.5).
// However, the school also issues alphabetical grades (A+ -> F-) and needs
// to be able to print both types of report card!

// Make the necessary code changes in the struct ReportCard and the impl block
// to support alphabetical report cards. Change the Grade in the second test to "A+"
// to show that your changes allow alphabetical grades.

// Execute `rustlings hint quiz3` or use the `hint` watch subcommand for a hint.


use std::fmt::Display;

enum Grade {
    Numeric(u32),
    Alphabetic(String),
}

pub struct ReportCard<T> {
    pub grade: T,
    pub student: Student,
}
// pub struct ReportCard{
//     pub grade: Grade,
//     pub student: Student,
// }

pub struct Student {
    pub name: String,
    pub age: u32,
}

impl<T: Display> ReportCard<T> {
    pub fn print(&self) -> String {
        format!("{} ({}) - achieved a grade of {}",
                &self.student.name, &self.student.age, &self.grade)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_numeric_report_card() {
        let student = Student {
            name: "Tom Wriggle".to_string(),
            age: 12,
        };

        let report_card = ReportCard {
            student,
            grade: 2.1,
        };
        assert_eq!(
            report_card.print(),
            "Tom Wriggle (12) - achieved a grade of 2.1"
        );
    }

    #[test]
    fn generate_alphabetic_report_card() {
        // TODO: Make sure to change the grade here after you finish the exercise.
        let student = Student {
            name: "Gary Plotter".to_string(),
            age: 11,
        };
        let report_card = ReportCard {
            student,
            grade: "A+",
        };
        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - achieved a grade of A+"
        );
    }
}
