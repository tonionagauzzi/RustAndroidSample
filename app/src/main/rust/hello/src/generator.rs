pub mod human {
    use array_macro::array;
    use chrono::{Duration, Utc};
    use rand::Rng;
    use std::ffi::CString;

    const DISABLE_INITIALS_FOR_MALE: [char; 7] = ['L', 'P', 'Q', 'V', 'W', 'X', 'Z'];
    const DISABLE_INITIALS_FOR_FEMALE: [char; 9] =
        ['D', 'G', 'L', 'P', 'Q', 'V', 'W', 'X', 'Z'];
    const DISABLE_INITIALS_FOR_LASTNAME: [char; 5] = ['L', 'P', 'Q', 'V', 'X'];

    // Hobbies list from https://www.lifehack.org/articles/money/this-list-50-low-cost-hobbies-will-excite-you-2.html
    const HOBBIES: [&str; 60] = [
        "DIY",
        "Documentaries",
        "Learning",
        "Gardening",
        "Camping",
        "Board Games",
        "Musics",
        "Knitting",
        "Cooking",
        "Drawing",
        "Investing",
        "Volunteer",
        "Yoga",
        "Writing",
        "Card Games",
        "Dancing",
        "Reading",
        "Language",
        "Magics",
        "Games",
        "Origami",
        "Internet Surfing",
        "Writing songs",
        "Visit museums",
        "Running",
        "Meditation",
        "Blogs",
        "Podcasts",
        "Photography",
        "Cycling",
        "Mentaling",
        "Programming",
        "Meet Friends",
        "Calligraphy",
        "Collection",
        "Watching People",
        "Fishing",
        "Traveling",
        "Driving",
        "Swimming",
        "Walking",
        "Movies",
        "Interiors",
        "Designing",
        "Fashion",
        "Shopping",
        "Baseball",
        "Soccer ball",
        "Tennis",
        "Table Tennis",
        "Volleyball",
        "Badminton",
        "Smart Devices",
        "Building Computers",
        "Mountaineering",
        "Visit Beaches",
        "Bouldering",
        "YouTube",
        "Twitter",
        "Instagram",
    ];    

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
        let zero_or_not = rng.gen_range(0..2);
        if zero_or_not == 0 { "Male" } else { "Female" }.to_string()
    }

    pub fn birthday() -> String {
        let mut rng = rand::thread_rng();
        let days = rng.gen_range(6667..20000);
        let now = Utc::now();
        (now - Duration::days(days)).format("%Y-%m-%d").to_string()
    }

    fn hobby() -> String {
        let mut rng = rand::thread_rng();
        HOBBIES[rng.gen_range(0..HOBBIES.len())].to_string()
    }

    pub fn hobbies() -> [String; 5] {
        let mut hobbies: [String; 5] = array!["".to_string(); 5];
        let mut i = 0;
        while i < 5 {
            let hobby = hobby();
            if !hobbies.contains(&hobby) {
                hobbies[i] = hobby;
                i += 1;
            }
        }
        hobbies
    }
}
