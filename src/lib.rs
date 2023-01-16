pub mod keygen {
    use std::char;
    use rand::{SeedableRng, Rng};
    use rand_regex;
    use rand_xorshift;
    use regex_syntax;
    use regex::Regex;

    /// All valid formats for generated key.
    #[derive(Debug)]
    pub enum Format {
        LowerCase,
        UpperCase,
        Numeric,
        Alphanumeric,
        Base64Url,
    }

    /// Input config structure.
    pub struct Config {
        pub length: usize,
        pub format: Format,
        pub seed: String,
        pub takes: usize,
    }

    /// Implementation for config structure. Ensures only valid format is given.
    impl Config {
        pub fn new(length: usize, format: String, seed: String, takes: usize) -> Config {
            let fmt = match format.to_lowercase().as_str() {
                "lowercase" => Format::LowerCase,
                "uppercase" => Format::UpperCase,
                "numeric" => Format::Numeric,
                "alphanumeric" => Format::Alphanumeric,
                "base64url" => Format::Base64Url,
                _ => panic!("Invalid format given. Use -h/--help to see list of valid formats.")
            };

            let len = match &fmt {
                Format::Base64Url => 64, // Ignore length if Base64Url is specified
                _ => length
            };

            Config {
                length: len,
                format: fmt,
                seed,
                takes,
            }
        }
    }

    /// Convert seed phrase into usable format.
    fn get_seed(s: &str) -> rand_xorshift::XorShiftRng {
        let s = if s.len() > 16 { &s[..16] }
            else { s };
        let mut seed_bytes = [0u8; 16];
        seed_bytes[..s.len()].copy_from_slice(s.as_bytes());
        rand_xorshift::XorShiftRng::from_seed(seed_bytes)
    }

    /// Get regex, formatted for deterministic keygen.
    fn get_regex_for_seed(format: &Format, length: usize) -> String {
        let accepted_chars = match format {
            Format::LowerCase => "[a-z]",
            Format::UpperCase => "[A-Z]",
            Format::Numeric => "[\\d]",
            Format::Alphanumeric => "[a-zA-Z\\d]",
            Format::Base64Url => "[\\w\\d-]",
        };
        format!(r"{}{{{}}}", accepted_chars, length)
    }

    /// Generate deterministic random key.
    pub fn generate_deterministic_random_key(config: &Config) -> Vec<String> {
        let mut rng = get_seed(&config.seed);

        let mut parser = regex_syntax::ParserBuilder::new().unicode(false).build();

        let regex = get_regex_for_seed(&config.format, config.length);
        let hir = parser.parse(&regex).unwrap();
        let gen = rand_regex::Regex::with_hir(hir, 100).expect("Unable to generate random key");

        (&mut rng).sample_iter(&gen).take(config.takes).collect::<Vec<String>>()
    }

    /// Get all ASCII characters from a given regex.
    fn get_chars_from_ascii(regex: &str) -> String {
        let mut characters: String = String::from("");
        for i in 45..128 {
            if let Some(c) = char::from_u32(i) {
                if Regex::new(regex).unwrap().is_match(&c.to_string()) {
                    characters.push(c);
                }
            };
        }
        characters
    }

    /// Get character set for a given Format.
    fn get_charset(format: &Format) -> String {
        let regex = match format {
            Format::LowerCase => r"[a-z]",
            Format::UpperCase => r"[A-Z]",
            Format::Numeric => r"[\d]",
            Format::Alphanumeric => r"[a-zA-Z\d]",
            Format::Base64Url => r"[\w\d-]",
        };
        get_chars_from_ascii(regex)
    }

    /// Generate a random key, non-deterministically.
    pub fn generate_random_key(config: &Config) -> Vec<String> {
        let len = match &config.format {
            Format::Base64Url => 64,
            _ => config.length,
        };
        let mut keys = Vec::<String>::new();
        let binding = get_charset(&config.format);
        let charset: &[u8] = binding.as_bytes();

        let mut rng = rand::thread_rng();

        for _ in 0..config.takes {
            keys.push((0..len)
                .map(|_| {
                    let index = rng.gen_range(0..charset.len());
                    charset[index] as char
                })
                .collect());
        }
        keys
    }
}
