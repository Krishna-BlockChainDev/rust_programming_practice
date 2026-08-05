// ============================================================
// DAY 20: MINI PROJECT — Student Records System
// Topic: Apply Structs + Enums + Vectors + Option + Pattern Matching
// Time: 30-45 minutes
// ============================================================
//
// PROJECT: A simple student management system that:
//   1. Stores students with their subjects and grades
//   2. Calculates GPA and letter grade
//   3. Finds top/bottom students
//   4. Filters by grade category
//   5. Shows class statistics
//
// CONCEPTS USED:
//   ✅ Day 11 — Ownership (passing structs correctly)
//   ✅ Day 12 — Borrowing (passing by reference)
//   ✅ Day 14 — Structs (Student, Subject)
//   ✅ Day 15 — impl / Methods
//   ✅ Day 16 — Enums (Grade, Department)
//   ✅ Day 17 — Pattern Matching
//   ✅ Day 18 — Option<T>
//   ✅ Day 19 — Vectors
//
// HOW TO RUN:
//   $ cargo run   (inside the Day-20 folder)
//
// ============================================================

// ── Enum Definitions ─────────────────────────────────────────

#[derive(Debug, PartialEq)]
enum Department {
    ComputerScience,
    Electrical,
    Mechanical,
    Civil,
}

#[derive(Debug, PartialEq, Clone)]
enum LetterGrade {
    APlus,
    A,
    B,
    C,
    D,
    F,
}

#[derive(Debug, PartialEq)]
enum AttendanceStatus {
    Present,
    Absent,
    Late,
}

// ── Struct Definitions ────────────────────────────────────────

#[derive(Debug)]
struct Subject {
    name: String,
    marks: u32,     // out of 100
    max_marks: u32,
}

#[derive(Debug)]
struct Student {
    id: u32,
    name: String,
    department: Department,
    subjects: Vec<Subject>,
    email: Option<String>,
}

// ── impl Blocks ───────────────────────────────────────────────

impl Subject {
    fn new(name: &str, marks: u32, max_marks: u32) -> Subject {
        Subject {
            name: String::from(name),
            marks,
            max_marks,
        }
    }

    fn percentage(&self) -> f64 {
        (self.marks as f64 / self.max_marks as f64) * 100.0
    }
}

impl Department {
    fn name(&self) -> &str {
        match self {
            Department::ComputerScience => "Computer Science",
            Department::Electrical      => "Electrical Engineering",
            Department::Mechanical      => "Mechanical Engineering",
            Department::Civil           => "Civil Engineering",
        }
    }
}

impl LetterGrade {
    fn from_percentage(p: f64) -> LetterGrade {
        match p as u32 {
            90..=100 => LetterGrade::APlus,
            80..=89  => LetterGrade::A,
            70..=79  => LetterGrade::B,
            60..=69  => LetterGrade::C,
            50..=59  => LetterGrade::D,
            _        => LetterGrade::F,
        }
    }

    fn label(&self) -> &str {
        match self {
            LetterGrade::APlus => "A+ (Distinction)",
            LetterGrade::A     => "A  (First Class)",
            LetterGrade::B     => "B  (Second Class)",
            LetterGrade::C     => "C  (Pass)",
            LetterGrade::D     => "D  (Below Average)",
            LetterGrade::F     => "F  (Fail)",
        }
    }
}

impl Student {
    fn new(id: u32, name: &str, dept: Department) -> Student {
        Student {
            id,
            name: String::from(name),
            department: dept,
            subjects: Vec::new(),
            email: None,
        }
    }

    fn with_email(mut self, email: &str) -> Student {
        self.email = Some(String::from(email));
        self
    }

    fn add_subject(&mut self, subject: Subject) {
        self.subjects.push(subject);
    }

    fn total_marks(&self) -> u32 {
        self.subjects.iter().map(|s| s.marks).sum()
    }

    fn total_max_marks(&self) -> u32 {
        self.subjects.iter().map(|s| s.max_marks).sum()
    }

    fn percentage(&self) -> f64 {
        if self.total_max_marks() == 0 { return 0.0; }
        (self.total_marks() as f64 / self.total_max_marks() as f64) * 100.0
    }

    fn letter_grade(&self) -> LetterGrade {
        LetterGrade::from_percentage(self.percentage())
    }

    fn is_passing(&self) -> bool {
        self.percentage() >= 50.0
    }

    fn best_subject(&self) -> Option<&Subject> {
        self.subjects.iter().max_by(|a, b| {
            a.percentage().partial_cmp(&b.percentage()).unwrap()
        })
    }

    fn weakest_subject(&self) -> Option<&Subject> {
        self.subjects.iter().min_by(|a, b| {
            a.percentage().partial_cmp(&b.percentage()).unwrap()
        })
    }

    fn print_report(&self) {
        println!("┌─────────────────────────────────────────┐");
        println!("│ Student Report                          │");
        println!("├─────────────────────────────────────────┤");
        println!("│ ID:         {:<30}│", self.id);
        println!("│ Name:       {:<30}│", self.name);
        println!("│ Dept:       {:<30}│", self.department.name());
        println!("│ Email:      {:<30}│",
            self.email.as_deref().unwrap_or("N/A"));
        println!("├─────────────────────────────────────────┤");
        println!("│ Subject Marks:                          │");
        for sub in &self.subjects {
            println!("│   {:20} {:3}/{:3}  ({:.1}%)    │",
                sub.name, sub.marks, sub.max_marks, sub.percentage());
        }
        println!("├─────────────────────────────────────────┤");
        println!("│ Total:  {}/{}", self.total_marks(), self.total_max_marks());
        println!("│ Percentage: {:.2}%", self.percentage());
        println!("│ Grade:      {}", self.letter_grade().label());
        println!("│ Status:     {}", if self.is_passing() { "PASS ✓" } else { "FAIL ✗" });

        if let Some(best) = self.best_subject() {
            println!("│ Best:       {} ({:.1}%)", best.name, best.percentage());
        }
        if let Some(worst) = self.weakest_subject() {
            println!("│ Weakest:    {} ({:.1}%)", worst.name, worst.percentage());
        }
        println!("└─────────────────────────────────────────┘");
    }
}

// ── Classroom/Statistics Functions ───────────────────────────

fn class_average(students: &[Student]) -> f64 {
    if students.is_empty() { return 0.0; }
    let total: f64 = students.iter().map(|s| s.percentage()).sum();
    total / students.len() as f64
}

fn top_student(students: &[Student]) -> Option<&Student> {
    students.iter().max_by(|a, b| {
        a.percentage().partial_cmp(&b.percentage()).unwrap()
    })
}

fn bottom_student(students: &[Student]) -> Option<&Student> {
    students.iter().min_by(|a, b| {
        a.percentage().partial_cmp(&b.percentage()).unwrap()
    })
}

fn grade_distribution(students: &[Student]) {
    let mut a_plus = 0; let mut a = 0; let mut b = 0;
    let mut c = 0; let mut d = 0; let mut f = 0;

    for s in students {
        match s.letter_grade() {
            LetterGrade::APlus => a_plus += 1,
            LetterGrade::A     => a += 1,
            LetterGrade::B     => b += 1,
            LetterGrade::C     => c += 1,
            LetterGrade::D     => d += 1,
            LetterGrade::F     => f += 1,
        }
    }

    println!("Grade Distribution:");
    println!("  A+: {} students", a_plus);
    println!("  A:  {} students", a);
    println!("  B:  {} students", b);
    println!("  C:  {} students", c);
    println!("  D:  {} students", d);
    println!("  F:  {} students", f);
}

fn separator() { println!("{}", "═".repeat(45)); }

// ─────────────────────────────────────────────────────────────

fn main() {
    separator();
    println!("   🎓 STUDENT RECORDS MANAGEMENT SYSTEM");
    separator();

    // ── Create Students ───────────────────────────────────────
    let mut students: Vec<Student> = Vec::new();

    let mut s1 = Student::new(1, "Krishna Rajput", Department::ComputerScience)
        .with_email("krishna@uni.edu");
    s1.add_subject(Subject::new("Mathematics",        88, 100));
    s1.add_subject(Subject::new("Data Structures",    92, 100));
    s1.add_subject(Subject::new("Operating Systems",  76, 100));
    s1.add_subject(Subject::new("Database Systems",   85, 100));
    s1.add_subject(Subject::new("Computer Networks",  79, 100));
    students.push(s1);

    let mut s2 = Student::new(2, "Alice Sharma", Department::Electrical)
        .with_email("alice@uni.edu");
    s2.add_subject(Subject::new("Circuit Theory",     95, 100));
    s2.add_subject(Subject::new("Signals & Systems",  88, 100));
    s2.add_subject(Subject::new("Electromagnetics",   91, 100));
    s2.add_subject(Subject::new("Control Systems",    87, 100));
    s2.add_subject(Subject::new("Power Electronics",  93, 100));
    students.push(s2);

    let mut s3 = Student::new(3, "Bob Mehta", Department::Mechanical);
    s3.add_subject(Subject::new("Thermodynamics",     62, 100));
    s3.add_subject(Subject::new("Fluid Mechanics",    58, 100));
    s3.add_subject(Subject::new("Machine Design",     70, 100));
    s3.add_subject(Subject::new("Manufacturing",      65, 100));
    s3.add_subject(Subject::new("Heat Transfer",      55, 100));
    students.push(s3);

    let mut s4 = Student::new(4, "Carol Gupta", Department::ComputerScience)
        .with_email("carol@uni.edu");
    s4.add_subject(Subject::new("Mathematics",        45, 100));
    s4.add_subject(Subject::new("Data Structures",    40, 100));
    s4.add_subject(Subject::new("Operating Systems",  38, 100));
    s4.add_subject(Subject::new("Database Systems",   52, 100));
    s4.add_subject(Subject::new("Computer Networks",  43, 100));
    students.push(s4);

    let mut s5 = Student::new(5, "David Patel", Department::Civil);
    s5.add_subject(Subject::new("Structural Analysis", 78, 100));
    s5.add_subject(Subject::new("Soil Mechanics",      72, 100));
    s5.add_subject(Subject::new("Fluid Mechanics",     80, 100));
    s5.add_subject(Subject::new("Transportation",      75, 100));
    s5.add_subject(Subject::new("Construction Mgmt",   68, 100));
    students.push(s5);

    // ── Individual Reports ────────────────────────────────────
    println!("\n📋 INDIVIDUAL STUDENT REPORTS");
    separator();
    for student in &students {
        student.print_report();
        println!();
    }

    // ── Class Statistics ─────────────────────────────────────
    println!("📊 CLASS STATISTICS");
    separator();
    println!("Total Students: {}", students.len());
    println!("Class Average:  {:.2}%", class_average(&students));

    let passing: Vec<&Student> = students.iter().filter(|s| s.is_passing()).collect();
    let failing: Vec<&Student> = students.iter().filter(|s| !s.is_passing()).collect();
    println!("Passing: {} students", passing.len());
    println!("Failing: {} students", failing.len());

    if let Some(top) = top_student(&students) {
        println!("\n🥇 Top Student: {} ({:.2}%)", top.name, top.percentage());
    }
    if let Some(bottom) = bottom_student(&students) {
        println!("📉 Lowest Score: {} ({:.2}%)", bottom.name, bottom.percentage());
    }

    println!();
    grade_distribution(&students);

    // ── Ranked List ────────────────────────────────────────────
    println!("\n🏆 CLASS RANKING");
    separator();
    let mut ranked: Vec<&Student> = students.iter().collect();
    ranked.sort_by(|a, b| b.percentage().partial_cmp(&a.percentage()).unwrap());

    for (i, s) in ranked.iter().enumerate() {
        println!("  {}. {:20} {:.2}%  {}",
            i + 1,
            s.name,
            s.percentage(),
            s.letter_grade().label()
        );
    }

    // ── Department Filter ─────────────────────────────────────
    println!("\n🖥️  COMPUTER SCIENCE STUDENTS");
    separator();
    let cs_students: Vec<&Student> = students.iter()
        .filter(|s| s.department == Department::ComputerScience)
        .collect();

    for s in &cs_students {
        println!("  {} — {:.2}% ({})", s.name, s.percentage(), s.letter_grade().label());
    }

    separator();
    println!("  Day 20 Mini Project COMPLETE! 🎉");
    println!("  Phase 2 DONE: Ownership → Structs → Enums → Vectors!");
    separator();

    // ── YOUR EXTENSION CHALLENGES ─────────────────────────────
    // 1. Add a `gpa: f64` calculation (4.0 scale):
    //    A+ = 4.0, A = 3.7, B = 3.3, C = 2.7, D = 2.0, F = 0.0

    // 2. Add an `attendance: Vec<AttendanceStatus>` field to Student
    //    Calculate attendance percentage and flag if < 75%

    // 3. Add a `search_by_name(students: &[Student], query: &str)` function
    //    that returns Vec<&Student> with matching names

    // 4. Add a method to export a student's data as CSV string:
    //    "id,name,dept,percentage,grade"
}
