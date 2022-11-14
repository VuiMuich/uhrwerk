use crate::{
    hours::Hours, minutes::Minutes, prepositions::Prepositions, special_cases::SpecialCases,
    template::Template,
};

impl Default for Template {
    fn default() -> Self {
        let language = String::from("default_deutsch");
        let minutes: Minutes = Minutes {
            five_past: String::from("f\u{fc}nf nach"),
            ten_past: String::from("zehn nach"),
            quarter_past: String::from("viertel nach"),
            twenty_past: String::from("zehn vor halb"),
            twenty_five_past: String::from("f\u{fc}nf vor halb"),
            half_past: String::from("halb"),
            twenty_five_to: String::from("f\u{fc}nf nach halb"),
            twenty_to: String::from("zehn nach halb"),
            quarter_to: String::from("dreiviertel"),
            ten_to: String::from("zehn vor"),
            five_to: String::from("f\u{fc}nf vor"),
            mini_err: String::from("Deine Minuten sind kaputt."),
        };

        let hours: Hours = Hours {
            one: String::from("eins"),
            two: String::from("zwei"),
            three: String::from("drei"),
            four: String::from("vier"),
            five: String::from("f\u{fc}nf"),
            six: String::from("sechs"),
            seven: String::from("sieben"),
            eight: String::from("acht"),
            nine: String::from("neun"),
            ten: String::from("zehn"),
            eleven: String::from("elf"),
            twelve: String::from("zw\u{f6}lf"),
            hour_err: String::from("Was ist mit deinen Stunden los?"),
        };
        let special_cases: SpecialCases = SpecialCases {
            before_midnight: String::from("Es ist gleich Mitternacht."),
            midnight: String::from("Es ist Mitternacht."),
            after_midnight: String::from("Es ist nach Mitternacht."),
            two_to_one: String::from("Es ist demn\u{e4}chst ein Uhr."),
            one_to_one: String::from("Es ist kurz vor ein Uhr."),
            exactly_one: String::from("Es ist ein Uhr."),
            one_past_one: String::from("Es ist kurz nach ein Uhr."),
            two_past_one: String::from("Es ist kurz nach ein Uhr."),
            high_noon: String::from("Es ist Mittag."),
        };
        let prepositions: Prepositions = Prepositions {
            almost: vec![
                String::from("gleich"),
                String::from("fast"),
                String::from("in Kl\u{fc}rze"),
                String::from("bald"),
                String::from("beinahe"),
            ],
            exactly: vec![
                String::from("exakt"),
                String::from("genau"),
                String::from("jetzt"),
                String::new(),
            ],
            roughly: vec![
                String::from("circa"),
                String::from("etwa"),
                String::from("ungef\u{e4}hr"),
            ],
            prepo_err: vec![String::from("Bitte was??")],
        };
        let start_sentence = vec![String::from("Es ist")];
        let end_sentence = vec![String::from("Uhr")];
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
