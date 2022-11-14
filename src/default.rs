use crate::{
    hours::Hours, minutes::Minutes, prepositions::Prepositions, special_cases::SpecialCases,
    template::Template,
};

impl Default for Template {
    fn default() -> Self {
        let language = String::from("default_english");
        let minutes: Minutes = Minutes {
            five_past: String::from("five past"),
            ten_past: String::from("ten past"),
            quarter_past: String::from("quarter past"),
            twenty_past: String::from("twenty past"),
            twenty_five_past: String::from("twenty-five past"),
            half_past: String::from("half past"),
            twenty_five_to: String::from("twenty-five to"),
            twenty_to: String::from("twenty to"),
            quarter_to: String::from("quarter to"),
            ten_to: String::from("ten to"),
            five_to: String::from("five to"),
            mini_err: String::from("minute error"),
        };

        let hours: Hours = Hours {
            one: String::from("one"),
            two: String::from("two"),
            three: String::from("three"),
            four: String::from("four"),
            five: String::from("five"),
            six: String::from("six"),
            seven: String::from("seven"),
            eight: String::from("eight"),
            nine: String::from("nine"),
            ten: String::from("ten"),
            eleven: String::from("eleven"),
            twelve: String::from("twelve"),
            hour_err: String::from("whats wrong with the hours"),
        };
        let special_cases: SpecialCases = SpecialCases {
            before_midnight: String::from("It’s ’round about midnight."),
            midnight: String::from("It’s midnight."),
            after_midnight: String::from("It’s ’round about midnight."),
            two_to_one: String::from("It’s two to one."),
            one_to_one: String::from("It’s one to one."),
            exactly_one: String::from("It’s exactly one."),
            one_past_one: String::from("It’s one past one."),
            two_past_one: String::from("It’s two past one."),
            high_noon: String::from("It’s noon."),
        };
        let prepositions: Prepositions = Prepositions {
            almost: vec![String::from("almost"), String::from("nearly")],
            exactly: vec![
                String::from("exactly"),
                String::from("precisely"),
                String::from("now"),
                String::new(),
            ],
            roughly: vec![
                String::from("just after"),
                String::from("right after"),
                String::from("shortly after"),
                String::from("about"),
                String::from("around"),
            ],
            prepo_err: vec![String::from("The what??")],
        };
        let start_sentence = vec![String::from("It's"), String::from("It is")];
        let end_sentence = vec![String::from("o'clock")];
        let on_the_hour_template = vec![
            String::from("start_sentence"),
            String::from("prepostition"),
            String::from("hour"),
            String::from("end_sentence"),
        ];
        let normal_template = vec![
            String::from("start_sentence"),
            String::from("prepostition"),
            String::from("minute"),
            String::from("hour"),
        ];
        //println!("Default template loaded.");
        Template {
            language,
            hours,
            minutes,
            prepositions,
            special_cases,
            start_sentence,
            end_sentence,
            on_the_hour_template,
            normal_template,
        }
    }
}
