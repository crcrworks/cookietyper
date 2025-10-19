use core::f64;

const THOUSAND_STR: &str = "thousand";
const MILLION_STR: &str = "million";
const BILLION_STR: &str = "billion";
const TRILLION_STR: &str = "trillion";
const QUADRILLION_STR: &str = "quadrillion";
const QUINTILLION_STR: &str = "quintillion";
const SEXTILLION_STR: &str = "sextillion";
const SEPTILLION_STR: &str = "septillion";
const OCTILLION_STR: &str = "octillion";
const NONILLION_STR: &str = "nonillion";
const DECILLION_STR: &str = "decillion";
const UNDECILLION_STR: &str = "undecillion";
const DUODECILLION_STR: &str = "duodecillion";
const TREDECILLION_STR: &str = "tredecillion";
const QUATTUORDECILLION_STR: &str = "quattuordecillion";
const QUINDECILLION_STR: &str = "quindecillion";
const SEXDECILLION_STR: &str = "sexdecillion";
const SEPTENDECILLION_STR: &str = "septendecillion";
const OCTODECILLION_STR: &str = "octodecillion";
const NOVEMDECILLION_STR: &str = "novemdecillion";
const VIGINTILLION_STR: &str = "vigintillion";
const UNVIGINTILLION_STR: &str = "unvigintillion";
const DUOVIGINTILLION_STR: &str = "duovigintillion";
const TREVIGINTILLION_STR: &str = "trevigintillion";
const QUATTUORVIGINTILLION_STR: &str = "quattuorvigintillion";
const QUINVIGINTILLION_STR: &str = "quinvigintillion";
const SEXVIGINTILLION_STR: &str = "sexvigintillion";
const SEPTENVIGINTILLION_STR: &str = "septenvigintillion";
const OCTOVIGINTILLION_STR: &str = "octovigintillion";
const NOVEMVIGINTILLION_STR: &str = "novemvigintillion";
const TRIGINTILLION_STR: &str = "trigintillion";
const UNTRIGINTILLION_STR: &str = "untrigintillion";
const DUOTRIGINTILLION_STR: &str = "duotrigintillion";
const TRETRIGINTILLION_STR: &str = "tretrigintillion";
const QUATTUORTRIGINTILLION_STR: &str = "quattuortrigintillion";
const QUINTRIGINTILLION_STR: &str = "quintrigintillion";
const SEXTRIGINTILLION_STR: &str = "sextrigintillion";
const SEPTENTRIGINTILLION_STR: &str = "septentrigintillion";
const OCTOTRIGINTILLION_STR: &str = "octotrigintillion";
const NOVEMTRIGINTILLION_STR: &str = "novemtrigintillion";
const QUADRAGINTILLION_STR: &str = "quadragintillion";
const UNQUADRAGINTILLION_STR: &str = "unquadragintillion";
const DUOQUADRAGINTILLION_STR: &str = "duoquadragintillion";
const TREQUADRAGINTILLION_STR: &str = "trequadragintillion";
const QUATTUORQUADRAGINTILLION_STR: &str = "quattuorquadragintillion";
const QUINQUADRAGINTILLION_STR: &str = "quinquadragintillion";
const SEXQUADRAGINTILLION_STR: &str = "sexquadragintillion";
const SEPTENQUADRAGINTILLION_STR: &str = "septenquadragintillion";
const OCTOQUADRAGINTILLION_STR: &str = "octoquadragintillion";
const NOVEMQUADRAGINTILLION_STR: &str = "novemquadragintillion";
const QUINQUAGINTILLION_STR: &str = "quinquagintillion";
const UNQUINQUAGINTILLION_STR: &str = "unquinquagintillion";
const DUOQUINQUAGINTILLION_STR: &str = "duoquinquagintillion";
const TREQUINQUAGINTILLION_STR: &str = "trequinquagintillion";
const QUATTUORQUINQUAGINTILLION_STR: &str = "quattuorquinquagintillion";
const QUINQUINQUAGINTILLION_STR: &str = "quinquinquagintillion";
const SEXQUINQUAGINTILLION_STR: &str = "sexquinquagintillion";
const SEPTENQUINQUAGINTILLION_STR: &str = "septenquinquagintillion";
const OCTOQUINQUAGINTILLION_STR: &str = "octoquinquagintillion";
const NOVEMQUINQUAGINTILLION_STR: &str = "novemquinquagintillion";
const SEXAGINTILLION_STR: &str = "sexagintillion";
const UNSEXAGINTILLION_STR: &str = "unsexagintillion";
const DUOSEXAGINTILLION_STR: &str = "duosexagintillion";
const TRESEXAGINTILLION_STR: &str = "tresexagintillion";
const QUATTUORSEXAGINTILLION_STR: &str = "quattuorsexagintillion";
const QUINSEXAGINTILLION_STR: &str = "quinsexagintillion";
const SEXSEXAGINTILLION_STR: &str = "sexsexagintillion";
const SEPTENSEXAGINTILLION_STR: &str = "septensexagintillion";
const OCTOSEXAGINTILLION_STR: &str = "octosexagintillion";
const NOVEMSEXAGINTILLION_STR: &str = "novemsexagintillion";
const SEPTUAGINTILLION_STR: &str = "septuagintillion";
const UNSEPTUAGINTILLION_STR: &str = "unseptuagintillion";
const DUOSEPTUAGINTILLION_STR: &str = "duoseptuagintillion";
const TRESEPTUAGINTILLION_STR: &str = "treseptuagintillion";
const QUATTUORSEPTUAGINTILLION_STR: &str = "quattuorseptuagintillion";
const QUINSEPTUAGINTILLION_STR: &str = "quinseptuagintillion";
const SEXSEPTUAGINTILLION_STR: &str = "sexseptuagintillion";
const SEPTENSEPTUAGINTILLION_STR: &str = "septenseptuagintillion";
const OCTOSEPTUAGINTILLION_STR: &str = "octoseptuagintillion";
const NOVEMSEPTUAGINTILLION_STR: &str = "novemseptuagintillion";
const OCTOGINTILLION_STR: &str = "octogintillion";
const UNOCTOGINTILLION_STR: &str = "unoctogintillion";
const DUOOCTOGINTILLION_STR: &str = "duooctogintillion";
const TREOCTOGINTILLION_STR: &str = "treoctogintillion";
const QUATTUOROCTOGINTILLION_STR: &str = "quattuoroctogintillion";
const QUINOCTOGINTILLION_STR: &str = "quinoctogintillion";
const SEXOCTOGINTILLION_STR: &str = "sexoctogintillion";
const SEPTENOCTOGINTILLION_STR: &str = "septenoctogintillion";
const OCTOOCTOGINTILLION_STR: &str = "octooctogintillion";
const NOVEMOCTOGINTILLION_STR: &str = "novemoctogintillion";
const NONAGINTILLION_STR: &str = "nonagintillion";
const UNNONAGINTILLION_STR: &str = "unnonagintillion";
const DUONONAGINTILLION_STR: &str = "duononagintillion";
const TRENONAGINTILLION_STR: &str = "trenonagintillion";
const QUATTUORNONAGINTILLION_STR: &str = "quattuornonagintillion";
const QUINNONAGINTILLION_STR: &str = "quinnonagintillion";
const SEXNONAGINTILLION_STR: &str = "sexnonagintillion";
const SEPTENNONAGINTILLION_STR: &str = "septennonagintillion";
const OCTONONAGINTILLION_STR: &str = "octononagintillion";

pub trait LingNum {
    const THOUSAND: f64 = 1e3;
    const MILLION: f64 = 1e6;
    const BILLION: f64 = 1e9;
    const TRILLION: f64 = 1e12;
    const QUADRILLION: f64 = 1e15;
    const QUINTILLION: f64 = 1e18;
    const SEXTILLION: f64 = 1e21;
    const SEPTILLION: f64 = 1e24;
    const OCTILLION: f64 = 1e27;
    const NONILLION: f64 = 1e30;
    const DECILLION: f64 = 1e33;
    const UNDECILLION: f64 = 1e36;
    const DUODECILLION: f64 = 1e39;
    const TREDECILLION: f64 = 1e42;
    const QUATTUORDECILLION: f64 = 1e45;
    const QUINDECILLION: f64 = 1e48;
    const SEXDECILLION: f64 = 1e51;
    const SEPTENDECILLION: f64 = 1e54;
    const OCTODECILLION: f64 = 1e57;
    const NOVEMDECILLION: f64 = 1e60;
    const VIGINTILLION: f64 = 1e63;
    const UNVIGINTILLION: f64 = 1e66;
    const DUOVIGINTILLION: f64 = 1e69;
    const TREVIGINTILLION: f64 = 1e72;
    const QUATTUORVIGINTILLION: f64 = 1e75;
    const QUINVIGINTILLION: f64 = 1e78;
    const SEXVIGINTILLION: f64 = 1e81;
    const SEPTENVIGINTILLION: f64 = 1e84;
    const OCTOVIGINTILLION: f64 = 1e87;
    const NOVEMVIGINTILLION: f64 = 1e90;
    const TRIGINTILLION: f64 = 1e93;
    const UNTRIGINTILLION: f64 = 1e96;
    const DUOTRIGINTILLION: f64 = 1e99;
    const TRETRIGINTILLION: f64 = 1e102;
    const QUATTUORTRIGINTILLION: f64 = 1e105;
    const QUINTRIGINTILLION: f64 = 1e108;
    const SEXTRIGINTILLION: f64 = 1e111;
    const SEPTENTRIGINTILLION: f64 = 1e114;
    const OCTOTRIGINTILLION: f64 = 1e117;
    const NOVEMTRIGINTILLION: f64 = 1e120;
    const QUADRAGINTILLION: f64 = 1e123;
    const UNQUADRAGINTILLION: f64 = 1e126;
    const DUOQUADRAGINTILLION: f64 = 1e129;
    const TREQUADRAGINTILLION: f64 = 1e132;
    const QUATTUORQUADRAGINTILLION: f64 = 1e135;
    const QUINQUADRAGINTILLION: f64 = 1e138;
    const SEXQUADRAGINTILLION: f64 = 1e141;
    const SEPTENQUADRAGINTILLION: f64 = 1e144;
    const OCTOQUADRAGINTILLION: f64 = 1e147;
    const NOVEMQUADRAGINTILLION: f64 = 1e150;
    const QUINQUAGINTILLION: f64 = 1e153;
    const UNQUINQUAGINTILLION: f64 = 1e156;
    const DUOQUINQUAGINTILLION: f64 = 1e159;
    const TREQUINQUAGINTILLION: f64 = 1e162;
    const QUATTUORQUINQUAGINTILLION: f64 = 1e165;
    const QUINQUINQUAGINTILLION: f64 = 1e168;
    const SEXQUINQUAGINTILLION: f64 = 1e171;
    const SEPTENQUINQUAGINTILLION: f64 = 1e174;
    const OCTOQUINQUAGINTILLION: f64 = 1e177;
    const NOVEMQUINQUAGINTILLION: f64 = 1e180;
    const SEXAGINTILLION: f64 = 1e183;
    const UNSEXAGINTILLION: f64 = 1e186;
    const DUOSEXAGINTILLION: f64 = 1e189;
    const TRESEXAGINTILLION: f64 = 1e192;
    const QUATTUORSEXAGINTILLION: f64 = 1e195;
    const QUINSEXAGINTILLION: f64 = 1e198;
    const SEXSEXAGINTILLION: f64 = 1e201;
    const SEPTENSEXAGINTILLION: f64 = 1e204;
    const OCTOSEXAGINTILLION: f64 = 1e207;
    const NOVEMSEXAGINTILLION: f64 = 1e210;
    const SEPTUAGINTILLION: f64 = 1e213;
    const UNSEPTUAGINTILLION: f64 = 1e216;
    const DUOSEPTUAGINTILLION: f64 = 1e219;
    const TRESEPTUAGINTILLION: f64 = 1e222;
    const QUATTUORSEPTUAGINTILLION: f64 = 1e225;
    const QUINSEPTUAGINTILLION: f64 = 1e228;
    const SEXSEPTUAGINTILLION: f64 = 1e231;
    const SEPTENSEPTUAGINTILLION: f64 = 1e234;
    const OCTOSEPTUAGINTILLION: f64 = 1e237;
    const NOVEMSEPTUAGINTILLION: f64 = 1e240;
    const OCTOGINTILLION: f64 = 1e243;
    const UNOCTOGINTILLION: f64 = 1e246;
    const DUOOCTOGINTILLION: f64 = 1e249;
    const TREOCTOGINTILLION: f64 = 1e252;
    const QUATTUOROCTOGINTILLION: f64 = 1e255;
    const QUINOCTOGINTILLION: f64 = 1e258;
    const SEXOCTOGINTILLION: f64 = 1e261;
    const SEPTENOCTOGINTILLION: f64 = 1e264;
    const OCTOOCTOGINTILLION: f64 = 1e267;
    const NOVEMOCTOGINTILLION: f64 = 1e270;
    const NONAGINTILLION: f64 = 1e273;
    const UNNONAGINTILLION: f64 = 1e276;
    const DUONONAGINTILLION: f64 = 1e279;
    const TRENONAGINTILLION: f64 = 1e282;
    const QUATTUORNONAGINTILLION: f64 = 1e285;
    const QUINNONAGINTILLION: f64 = 1e288;
    const SEXNONAGINTILLION: f64 = 1e291;
    const SEPTENNONAGINTILLION: f64 = 1e294;
    const OCTONONAGINTILLION: f64 = 1e297;

    fn lingual(self) -> String;
    fn lingual_without_thousand(self) -> String;
}

impl LingNum for f64 {
    fn lingual(self) -> String {
        let units = [
            (Self::OCTONONAGINTILLION, OCTONONAGINTILLION_STR),
            (Self::SEPTENNONAGINTILLION, SEPTENNONAGINTILLION_STR),
            (Self::SEXNONAGINTILLION, SEXNONAGINTILLION_STR),
            (Self::QUINNONAGINTILLION, QUINNONAGINTILLION_STR),
            (Self::QUATTUORNONAGINTILLION, QUATTUORNONAGINTILLION_STR),
            (Self::TRENONAGINTILLION, TRENONAGINTILLION_STR),
            (Self::DUONONAGINTILLION, DUONONAGINTILLION_STR),
            (Self::UNNONAGINTILLION, UNNONAGINTILLION_STR),
            (Self::NONAGINTILLION, NONAGINTILLION_STR),
            (Self::NOVEMOCTOGINTILLION, NOVEMOCTOGINTILLION_STR),
            (Self::OCTOOCTOGINTILLION, OCTOOCTOGINTILLION_STR),
            (Self::SEPTENOCTOGINTILLION, SEPTENOCTOGINTILLION_STR),
            (Self::SEXOCTOGINTILLION, SEXOCTOGINTILLION_STR),
            (Self::QUINOCTOGINTILLION, QUINOCTOGINTILLION_STR),
            (Self::QUATTUOROCTOGINTILLION, QUATTUOROCTOGINTILLION_STR),
            (Self::TREOCTOGINTILLION, TREOCTOGINTILLION_STR),
            (Self::DUOOCTOGINTILLION, DUOOCTOGINTILLION_STR),
            (Self::UNOCTOGINTILLION, UNOCTOGINTILLION_STR),
            (Self::OCTOGINTILLION, OCTOGINTILLION_STR),
            (Self::NOVEMSEPTUAGINTILLION, NOVEMSEPTUAGINTILLION_STR),
            (Self::OCTOSEPTUAGINTILLION, OCTOSEPTUAGINTILLION_STR),
            (Self::SEPTENSEPTUAGINTILLION, SEPTENSEPTUAGINTILLION_STR),
            (Self::SEXSEPTUAGINTILLION, SEXSEPTUAGINTILLION_STR),
            (Self::QUINSEPTUAGINTILLION, QUINSEPTUAGINTILLION_STR),
            (Self::QUATTUORSEPTUAGINTILLION, QUATTUORSEPTUAGINTILLION_STR),
            (Self::TRESEPTUAGINTILLION, TRESEPTUAGINTILLION_STR),
            (Self::DUOSEPTUAGINTILLION, DUOSEPTUAGINTILLION_STR),
            (Self::UNSEPTUAGINTILLION, UNSEPTUAGINTILLION_STR),
            (Self::SEPTUAGINTILLION, SEPTUAGINTILLION_STR),
            (Self::NOVEMSEXAGINTILLION, NOVEMSEXAGINTILLION_STR),
            (Self::OCTOSEXAGINTILLION, OCTOSEXAGINTILLION_STR),
            (Self::SEPTENSEXAGINTILLION, SEPTENSEXAGINTILLION_STR),
            (Self::SEXSEXAGINTILLION, SEXSEXAGINTILLION_STR),
            (Self::QUINSEXAGINTILLION, QUINSEXAGINTILLION_STR),
            (Self::QUATTUORSEXAGINTILLION, QUATTUORSEXAGINTILLION_STR),
            (Self::TRESEXAGINTILLION, TRESEXAGINTILLION_STR),
            (Self::DUOSEXAGINTILLION, DUOSEXAGINTILLION_STR),
            (Self::UNSEXAGINTILLION, UNSEXAGINTILLION_STR),
            (Self::SEXAGINTILLION, SEXAGINTILLION_STR),
            (Self::NOVEMQUINQUAGINTILLION, NOVEMQUINQUAGINTILLION_STR),
            (Self::OCTOQUINQUAGINTILLION, OCTOQUINQUAGINTILLION_STR),
            (Self::SEPTENQUINQUAGINTILLION, SEPTENQUINQUAGINTILLION_STR),
            (Self::SEXQUINQUAGINTILLION, SEXQUINQUAGINTILLION_STR),
            (Self::QUINQUINQUAGINTILLION, QUINQUINQUAGINTILLION_STR),
            (
                Self::QUATTUORQUINQUAGINTILLION,
                QUATTUORQUINQUAGINTILLION_STR,
            ),
            (Self::TREQUINQUAGINTILLION, TREQUINQUAGINTILLION_STR),
            (Self::DUOQUINQUAGINTILLION, DUOQUINQUAGINTILLION_STR),
            (Self::UNQUINQUAGINTILLION, UNQUINQUAGINTILLION_STR),
            (Self::QUINQUAGINTILLION, QUINQUAGINTILLION_STR),
            (Self::NOVEMQUADRAGINTILLION, NOVEMQUADRAGINTILLION_STR),
            (Self::OCTOQUADRAGINTILLION, OCTOQUADRAGINTILLION_STR),
            (Self::SEPTENQUADRAGINTILLION, SEPTENQUADRAGINTILLION_STR),
            (Self::SEXQUADRAGINTILLION, SEXQUADRAGINTILLION_STR),
            (Self::QUINQUADRAGINTILLION, QUINQUADRAGINTILLION_STR),
            (Self::QUATTUORQUADRAGINTILLION, QUATTUORQUADRAGINTILLION_STR),
            (Self::TREQUADRAGINTILLION, TREQUADRAGINTILLION_STR),
            (Self::DUOQUADRAGINTILLION, DUOQUADRAGINTILLION_STR),
            (Self::UNQUADRAGINTILLION, UNQUADRAGINTILLION_STR),
            (Self::QUADRAGINTILLION, QUADRAGINTILLION_STR),
            (Self::NOVEMTRIGINTILLION, NOVEMTRIGINTILLION_STR),
            (Self::OCTOTRIGINTILLION, OCTOTRIGINTILLION_STR),
            (Self::SEPTENTRIGINTILLION, SEPTENTRIGINTILLION_STR),
            (Self::SEXTRIGINTILLION, SEXTRIGINTILLION_STR),
            (Self::QUINTRIGINTILLION, QUINTRIGINTILLION_STR),
            (Self::QUATTUORTRIGINTILLION, QUATTUORTRIGINTILLION_STR),
            (Self::TRETRIGINTILLION, TRETRIGINTILLION_STR),
            (Self::DUOTRIGINTILLION, DUOTRIGINTILLION_STR),
            (Self::UNTRIGINTILLION, UNTRIGINTILLION_STR),
            (Self::TRIGINTILLION, TRIGINTILLION_STR),
            (Self::NOVEMVIGINTILLION, NOVEMVIGINTILLION_STR),
            (Self::OCTOVIGINTILLION, OCTOVIGINTILLION_STR),
            (Self::SEPTENVIGINTILLION, SEPTENVIGINTILLION_STR),
            (Self::SEXVIGINTILLION, SEXVIGINTILLION_STR),
            (Self::QUINVIGINTILLION, QUINVIGINTILLION_STR),
            (Self::QUATTUORVIGINTILLION, QUATTUORVIGINTILLION_STR),
            (Self::TREVIGINTILLION, TREVIGINTILLION_STR),
            (Self::DUOVIGINTILLION, DUOVIGINTILLION_STR),
            (Self::UNVIGINTILLION, UNVIGINTILLION_STR),
            (Self::VIGINTILLION, VIGINTILLION_STR),
            (Self::NOVEMDECILLION, NOVEMDECILLION_STR),
            (Self::OCTODECILLION, OCTODECILLION_STR),
            (Self::SEPTENDECILLION, SEPTENDECILLION_STR),
            (Self::SEXDECILLION, SEXDECILLION_STR),
            (Self::QUINDECILLION, QUINDECILLION_STR),
            (Self::QUATTUORDECILLION, QUATTUORDECILLION_STR),
            (Self::TREDECILLION, TREDECILLION_STR),
            (Self::DUODECILLION, DUODECILLION_STR),
            (Self::UNDECILLION, UNDECILLION_STR),
            (Self::DECILLION, DECILLION_STR),
            (Self::NONILLION, NONILLION_STR),
            (Self::OCTILLION, OCTILLION_STR),
            (Self::SEPTILLION, SEPTILLION_STR),
            (Self::SEXTILLION, SEXTILLION_STR),
            (Self::QUINTILLION, QUINTILLION_STR),
            (Self::QUADRILLION, QUADRILLION_STR),
            (Self::TRILLION, TRILLION_STR),
            (Self::BILLION, BILLION_STR),
            (Self::MILLION, MILLION_STR),
            (Self::THOUSAND, THOUSAND_STR),
        ];

        let abs_value = self.abs();

        if abs_value < 1000.0 {
            return self.trunc().to_string();
        }

        for (unit_value, unit_name) in units.iter() {
            if abs_value >= *unit_value {
                let result = self / unit_value;
                let s = format!("{:.3}", result);
                return format!("{} {}", s, unit_name);
            }
        }

        self.trunc().to_string()
    }

    fn lingual_without_thousand(self) -> String {
        let units = [
            (Self::OCTONONAGINTILLION, OCTONONAGINTILLION_STR),
            (Self::SEPTENNONAGINTILLION, SEPTENNONAGINTILLION_STR),
            (Self::SEXNONAGINTILLION, SEXNONAGINTILLION_STR),
            (Self::QUINNONAGINTILLION, QUINNONAGINTILLION_STR),
            (Self::QUATTUORNONAGINTILLION, QUATTUORNONAGINTILLION_STR),
            (Self::TRENONAGINTILLION, TRENONAGINTILLION_STR),
            (Self::DUONONAGINTILLION, DUONONAGINTILLION_STR),
            (Self::UNNONAGINTILLION, UNNONAGINTILLION_STR),
            (Self::NONAGINTILLION, NONAGINTILLION_STR),
            (Self::NOVEMOCTOGINTILLION, NOVEMOCTOGINTILLION_STR),
            (Self::OCTOOCTOGINTILLION, OCTOOCTOGINTILLION_STR),
            (Self::SEPTENOCTOGINTILLION, SEPTENOCTOGINTILLION_STR),
            (Self::SEXOCTOGINTILLION, SEXOCTOGINTILLION_STR),
            (Self::QUINOCTOGINTILLION, QUINOCTOGINTILLION_STR),
            (Self::QUATTUOROCTOGINTILLION, QUATTUOROCTOGINTILLION_STR),
            (Self::TREOCTOGINTILLION, TREOCTOGINTILLION_STR),
            (Self::DUOOCTOGINTILLION, DUOOCTOGINTILLION_STR),
            (Self::UNOCTOGINTILLION, UNOCTOGINTILLION_STR),
            (Self::OCTOGINTILLION, OCTOGINTILLION_STR),
            (Self::NOVEMSEPTUAGINTILLION, NOVEMSEPTUAGINTILLION_STR),
            (Self::OCTOSEPTUAGINTILLION, OCTOSEPTUAGINTILLION_STR),
            (Self::SEPTENSEPTUAGINTILLION, SEPTENSEPTUAGINTILLION_STR),
            (Self::SEXSEPTUAGINTILLION, SEXSEPTUAGINTILLION_STR),
            (Self::QUINSEPTUAGINTILLION, QUINSEPTUAGINTILLION_STR),
            (Self::QUATTUORSEPTUAGINTILLION, QUATTUORSEPTUAGINTILLION_STR),
            (Self::TRESEPTUAGINTILLION, TRESEPTUAGINTILLION_STR),
            (Self::DUOSEPTUAGINTILLION, DUOSEPTUAGINTILLION_STR),
            (Self::UNSEPTUAGINTILLION, UNSEPTUAGINTILLION_STR),
            (Self::SEPTUAGINTILLION, SEPTUAGINTILLION_STR),
            (Self::NOVEMSEXAGINTILLION, NOVEMSEXAGINTILLION_STR),
            (Self::OCTOSEXAGINTILLION, OCTOSEXAGINTILLION_STR),
            (Self::SEPTENSEXAGINTILLION, SEPTENSEXAGINTILLION_STR),
            (Self::SEXSEXAGINTILLION, SEXSEXAGINTILLION_STR),
            (Self::QUINSEXAGINTILLION, QUINSEXAGINTILLION_STR),
            (Self::QUATTUORSEXAGINTILLION, QUATTUORSEXAGINTILLION_STR),
            (Self::TRESEXAGINTILLION, TRESEXAGINTILLION_STR),
            (Self::DUOSEXAGINTILLION, DUOSEXAGINTILLION_STR),
            (Self::UNSEXAGINTILLION, UNSEXAGINTILLION_STR),
            (Self::SEXAGINTILLION, SEXAGINTILLION_STR),
            (Self::NOVEMQUINQUAGINTILLION, NOVEMQUINQUAGINTILLION_STR),
            (Self::OCTOQUINQUAGINTILLION, OCTOQUINQUAGINTILLION_STR),
            (Self::SEPTENQUINQUAGINTILLION, SEPTENQUINQUAGINTILLION_STR),
            (Self::SEXQUINQUAGINTILLION, SEXQUINQUAGINTILLION_STR),
            (Self::QUINQUINQUAGINTILLION, QUINQUINQUAGINTILLION_STR),
            (
                Self::QUATTUORQUINQUAGINTILLION,
                QUATTUORQUINQUAGINTILLION_STR,
            ),
            (Self::TREQUINQUAGINTILLION, TREQUINQUAGINTILLION_STR),
            (Self::DUOQUINQUAGINTILLION, DUOQUINQUAGINTILLION_STR),
            (Self::UNQUINQUAGINTILLION, UNQUINQUAGINTILLION_STR),
            (Self::QUINQUAGINTILLION, QUINQUAGINTILLION_STR),
            (Self::NOVEMQUADRAGINTILLION, NOVEMQUADRAGINTILLION_STR),
            (Self::OCTOQUADRAGINTILLION, OCTOQUADRAGINTILLION_STR),
            (Self::SEPTENQUADRAGINTILLION, SEPTENQUADRAGINTILLION_STR),
            (Self::SEXQUADRAGINTILLION, SEXQUADRAGINTILLION_STR),
            (Self::QUINQUADRAGINTILLION, QUINQUADRAGINTILLION_STR),
            (Self::QUATTUORQUADRAGINTILLION, QUATTUORQUADRAGINTILLION_STR),
            (Self::TREQUADRAGINTILLION, TREQUADRAGINTILLION_STR),
            (Self::DUOQUADRAGINTILLION, DUOQUADRAGINTILLION_STR),
            (Self::UNQUADRAGINTILLION, UNQUADRAGINTILLION_STR),
            (Self::QUADRAGINTILLION, QUADRAGINTILLION_STR),
            (Self::NOVEMTRIGINTILLION, NOVEMTRIGINTILLION_STR),
            (Self::OCTOTRIGINTILLION, OCTOTRIGINTILLION_STR),
            (Self::SEPTENTRIGINTILLION, SEPTENTRIGINTILLION_STR),
            (Self::SEXTRIGINTILLION, SEXTRIGINTILLION_STR),
            (Self::QUINTRIGINTILLION, QUINTRIGINTILLION_STR),
            (Self::QUATTUORTRIGINTILLION, QUATTUORTRIGINTILLION_STR),
            (Self::TRETRIGINTILLION, TRETRIGINTILLION_STR),
            (Self::DUOTRIGINTILLION, DUOTRIGINTILLION_STR),
            (Self::UNTRIGINTILLION, UNTRIGINTILLION_STR),
            (Self::TRIGINTILLION, TRIGINTILLION_STR),
            (Self::NOVEMVIGINTILLION, NOVEMVIGINTILLION_STR),
            (Self::OCTOVIGINTILLION, OCTOVIGINTILLION_STR),
            (Self::SEPTENVIGINTILLION, SEPTENVIGINTILLION_STR),
            (Self::SEXVIGINTILLION, SEXVIGINTILLION_STR),
            (Self::QUINVIGINTILLION, QUINVIGINTILLION_STR),
            (Self::QUATTUORVIGINTILLION, QUATTUORVIGINTILLION_STR),
            (Self::TREVIGINTILLION, TREVIGINTILLION_STR),
            (Self::DUOVIGINTILLION, DUOVIGINTILLION_STR),
            (Self::UNVIGINTILLION, UNVIGINTILLION_STR),
            (Self::VIGINTILLION, VIGINTILLION_STR),
            (Self::NOVEMDECILLION, NOVEMDECILLION_STR),
            (Self::OCTODECILLION, OCTODECILLION_STR),
            (Self::SEPTENDECILLION, SEPTENDECILLION_STR),
            (Self::SEXDECILLION, SEXDECILLION_STR),
            (Self::QUINDECILLION, QUINDECILLION_STR),
            (Self::QUATTUORDECILLION, QUATTUORDECILLION_STR),
            (Self::TREDECILLION, TREDECILLION_STR),
            (Self::DUODECILLION, DUODECILLION_STR),
            (Self::UNDECILLION, UNDECILLION_STR),
            (Self::DECILLION, DECILLION_STR),
            (Self::NONILLION, NONILLION_STR),
            (Self::OCTILLION, OCTILLION_STR),
            (Self::SEPTILLION, SEPTILLION_STR),
            (Self::SEXTILLION, SEXTILLION_STR),
            (Self::QUINTILLION, QUINTILLION_STR),
            (Self::QUADRILLION, QUADRILLION_STR),
            (Self::TRILLION, TRILLION_STR),
            (Self::BILLION, BILLION_STR),
            (Self::MILLION, MILLION_STR),
        ];

        let abs_value = self.abs();

        if abs_value < 1000.0 {
            return self.trunc().to_string();
        }

        for (unit_value, unit_name) in units.iter() {
            if abs_value >= *unit_value {
                let result = self / unit_value;
                let s = format!("{:.3}", result);
                return format!("{} {}", s, unit_name);
            }
        }

        self.trunc().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lingual_small_numbers() {
        let result = 500.0.lingual();
        assert_eq!(result, "500");
    }

    #[test]
    fn test_lingual_thousand() {
        let result = 1500.0.lingual();
        assert_eq!(result, "1.500 thousand");
    }

    #[test]
    fn test_lingual_million() {
        let result = 2500000.0.lingual();
        assert_eq!(result, "2.500 million");
    }

    #[test]
    fn test_lingual_billion() {
        let result = 3750000000.0.lingual();
        assert_eq!(result, "3.750 billion");
    }

    #[test]
    fn test_lingual_trillion() {
        let result = 1e13.lingual();
        assert_eq!(result, "10.000 trillion");
    }

    #[test]
    fn test_lingual_negative_numbers() {
        let result = (-1500.0).lingual();
        assert_eq!(result, "-1.500 thousand");
    }

    #[test]
    fn test_lingual_zero() {
        let result = 0.0.lingual();
        assert_eq!(result, "0");
    }

    #[test]
    fn test_lingual_large_units() {
        let result = 1e100.lingual();
        assert_eq!(result, "10.000 duotrigintillion");
    }

    #[test]
    fn test_lingual_very_large_units() {
        let result = 1e297.lingual();
        assert_eq!(result, "1.000 octononagintillion");
    }

    #[test]
    fn test_lingual_very_long_float() {
        let result = 952332.012331.lingual();
        assert_eq!(result, "952.332 thousand");
    }

    #[test]
    fn test_lingual_very_long_and_big_float() {
        let result = 952_332_213.012331.lingual();
        assert_eq!(result, "952.332 million");
    }

    #[test]
    fn test_lingual_very_long_float_end_with_zero() {
        let result = 952300.012331.lingual();
        assert_eq!(result, "952.300 thousand");
    }

    #[test]
    fn test_constants() {
        assert_eq!(f64::THOUSAND, 1e3);
        assert_eq!(f64::MILLION, 1e6);
        assert_eq!(f64::BILLION, 1e9);
        assert_eq!(f64::TRILLION, 1e12);
    }

    #[test]
    fn test_lingual_without_thousand_small_numbers() {
        let result = 500.0.lingual_without_thousand();
        assert_eq!(result, "500");
    }

    #[test]
    fn test_lingual_without_thousand_in_thousand_range() {
        let result = 1500.0.lingual_without_thousand();
        assert_eq!(result, "1500");
    }

    #[test]
    fn test_lingual_without_thousand_large_thousand_range() {
        let result = 999999.0.lingual_without_thousand();
        assert_eq!(result, "999999");
    }

    #[test]
    fn test_lingual_without_thousand_million() {
        let result = 2500000.0.lingual_without_thousand();
        assert_eq!(result, "2.500 million");
    }

    #[test]
    fn test_lingual_without_thousand_billion() {
        let result = 3750000000.0.lingual_without_thousand();
        assert_eq!(result, "3.750 billion");
    }

    #[test]
    fn test_lingual_without_thousand_trillion() {
        let result = 1e13.lingual_without_thousand();
        assert_eq!(result, "10.000 trillion");
    }

    #[test]
    fn test_lingual_without_thousand_negative() {
        let result = (-1500.0).lingual_without_thousand();
        assert_eq!(result, "-1500");
    }

    #[test]
    fn test_lingual_without_thousand_edge_case() {
        let result = 999999.999.lingual_without_thousand();
        assert_eq!(result, "999999");
    }
}
