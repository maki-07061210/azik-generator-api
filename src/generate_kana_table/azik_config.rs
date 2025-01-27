use std::str::FromStr;

use super::{
    assignable_tokens::{Assignable, AssignableTokens},
    gen_consonant::gen_consonants_array,
    gen_kana::gen_hiragana,
    gen_vowel::{Vowels, vowel_to_kana},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct AzikConfig {
    pub Sequence: Vec<Assignable>,
    pub Sokuon: String,
    pub Hatsuon: String,
}

pub fn gen_sequence(config: AzikConfig) -> String {
    let mut out = String::new();
    let hatsuon_sequence = AssignableTokens::from_str(config.Hatsuon.to_uppercase().as_str())
        .expect("It's not Assignable. Did you use vowels or numbers? change Hatsuon in JSON")
        .to_string()
        .to_lowercase()
        + "	"
        + "ん\n";

    out.push_str(&hatsuon_sequence.to_string());

    let sokuon_sequence = AssignableTokens::from_str(config.Sokuon.to_uppercase().as_str())
        .expect("It's not Assignable. Did you use vowels or numbers? change Sokuon in JSON")
        .to_string()
        .to_lowercase()
        + "	"
        + "っ\n\n";

    out.push_str(&sokuon_sequence.to_string());

    for i in config.Sequence {
        let sequence: Vec<_> = i.Sequence.to_string().chars().collect();
        let kana_vowel = Vowels::from_str(&sequence[0].to_uppercase().to_string().as_str());
        let free_vowel = vowel_to_kana(
            Vowels::from_str(&sequence[1].to_uppercase().to_string().as_str())
                .expect("It's not vowel or something"),
        );

        for j in gen_consonants_array() {
            let consonant = j;
            let assignable = i.Token.to_lowercase();
            let kana = gen_hiragana(consonant, kana_vowel.expect("It's not vowel or something"));

            out.push_str(
                &(consonant.to_string().to_lowercase()
                    + &assignable
                    + "	"
                    + kana
                    + free_vowel
                    + "\n"),
            );
        }
        out.push_str("\n");
    }

    out
}
