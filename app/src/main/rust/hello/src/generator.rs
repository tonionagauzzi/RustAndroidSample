pub mod human {
    use chrono::{Duration, Utc};
    use rand::Rng;
    use std::ffi::CString;

    const DISABLE_INITIALS_FOR_MALE: [char; 7] = ['L', 'P', 'Q', 'V', 'W', 'X', 'Z'];
    const DISABLE_INITIALS_FOR_FEMALE: [char; 11] =
        ['B', 'D', 'G', 'J', 'L', 'P', 'Q', 'V', 'W', 'X', 'Z'];
    const DISABLE_INITIALS_FOR_LASTNAME: [char; 5] = ['L', 'P', 'Q', 'V', 'X'];

    pub fn first_name(gender: CString) -> String {
        let mut rng = rand::thread_rng();
        let male = CString::new("Male").unwrap();
        loop {
            let initial = rng.gen_range(65..91) as u8 as char;
            if gender.eq(&male) && !DISABLE_INITIALS_FOR_MALE.contains(&initial)
                || !DISABLE_INITIALS_FOR_FEMALE.contains(&initial)
            {
                return initial.to_string();
            }
        }
    }

    pub fn last_name() -> String {
        let mut rng = rand::thread_rng();
        loop {
            let initial = rng.gen_range(65..91) as u8 as char;
            if !DISABLE_INITIALS_FOR_LASTNAME.contains(&initial) {
                return initial.to_string();
            }
        }
    }

    pub fn gender() -> String {
        let mut rng = rand::thread_rng();
        let range = rng.gen_range(0..2);
        if range == 0 { "Male" } else { "Female" }.to_string()
    }

    pub fn birthday() -> String {
        let mut rng = rand::thread_rng();
        let range = rng.gen_range(6667..20000);
        let now = Utc::now();
        (now - Duration::days(range)).format("%Y-%m-%d").to_string()
    }
}
