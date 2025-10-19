pub struct LingNum {
    inner: f64,
    should_use_thousand: bool,
}

impl LingNum {
    pub const THOUSAND: (f64, &'static str) = (1e3, "thousand");
    pub const MILLION: (f64, &'static str) = (1e6, "million");
    pub const BILLION: (f64, &'static str) = (1e9, "billion");
    pub const TRILLION: (f64, &'static str) = (1e12, "trillion");
    pub const QUADRILLION: (f64, &'static str) = (1e15, "quadrillion");
    pub const QUINTILLION: (f64, &'static str) = (1e18, "quintillion");
    pub const SEXTILLION: (f64, &'static str) = (1e21, "sextillion");
    pub const SEPTILLION: (f64, &'static str) = (1e24, "septillion");
    pub const OCTILLION: (f64, &'static str) = (1e27, "octillion");
    pub const NONILLION: (f64, &'static str) = (1e30, "nonillion");
    pub const DECILLION: (f64, &'static str) = (1e33, "decillion");
    pub const UNDECILLION: (f64, &'static str) = (1e36, "undecillion");
    pub const DUODECILLION: (f64, &'static str) = (1e39, "duodecillion");
    pub const TREDECILLION: (f64, &'static str) = (1e42, "tredecillion");
    pub const QUATTUORDECILLION: (f64, &'static str) = (1e45, "quattuordecillion");
    pub const QUINDECILLION: (f64, &'static str) = (1e48, "quindecillion");
    pub const SEXDECILLION: (f64, &'static str) = (1e51, "sexdecillion");
    pub const SEPTENDECILLION: (f64, &'static str) = (1e54, "septendecillion");
    pub const OCTODECILLION: (f64, &'static str) = (1e57, "octodecillion");
    pub const NOVEMDECILLION: (f64, &'static str) = (1e60, "novemdecillion");
    pub const VIGINTILLION: (f64, &'static str) = (1e63, "vigintillion");
    pub const UNVIGINTILLION: (f64, &'static str) = (1e66, "unvigintillion");
    pub const DUOVIGINTILLION: (f64, &'static str) = (1e69, "duovigintillion");
    pub const TREVIGINTILLION: (f64, &'static str) = (1e72, "trevigintillion");
    pub const QUATTUORVIGINTILLION: (f64, &'static str) = (1e75, "quattuorvigintillion");
    pub const QUINVIGINTILLION: (f64, &'static str) = (1e78, "quinvigintillion");
    pub const SEXVIGINTILLION: (f64, &'static str) = (1e81, "sexvigintillion");
    pub const SEPTENVIGINTILLION: (f64, &'static str) = (1e84, "septenvigintillion");
    pub const OCTOVIGINTILLION: (f64, &'static str) = (1e87, "octovigintillion");
    pub const NOVEMVIGINTILLION: (f64, &'static str) = (1e90, "novemvigintillion");
    pub const TRIGINTILLION: (f64, &'static str) = (1e93, "trigintillion");
    pub const UNTRIGINTILLION: (f64, &'static str) = (1e96, "untrigintillion");
    pub const DUOTRIGINTILLION: (f64, &'static str) = (1e99, "duotrigintillion");
    pub const TRETRIGINTILLION: (f64, &'static str) = (1e102, "tretrigintillion");
    pub const QUATTUORTRIGINTILLION: (f64, &'static str) = (1e105, "quattuortrigintillion");
    pub const QUINTRIGINTILLION: (f64, &'static str) = (1e108, "quintrigintillion");
    pub const SEXTRIGINTILLION: (f64, &'static str) = (1e111, "sextrigintillion");
    pub const SEPTENTRIGINTILLION: (f64, &'static str) = (1e114, "septentrigintillion");
    pub const OCTOTRIGINTILLION: (f64, &'static str) = (1e117, "octotrigintillion");
    pub const NOVEMTRIGINTILLION: (f64, &'static str) = (1e120, "novemtrigintillion");
    pub const QUADRAGINTILLION: (f64, &'static str) = (1e123, "quadragintillion");
    pub const UNQUADRAGINTILLION: (f64, &'static str) = (1e126, "unquadragintillion");
    pub const DUOQUADRAGINTILLION: (f64, &'static str) = (1e129, "duoquadragintillion");
    pub const TREQUADRAGINTILLION: (f64, &'static str) = (1e132, "trequadragintillion");
    pub const QUATTUORQUADRAGINTILLION: (f64, &'static str) = (1e135, "quattuorquadragintillion");
    pub const QUINQUADRAGINTILLION: (f64, &'static str) = (1e138, "quinquadragintillion");
    pub const SEXQUADRAGINTILLION: (f64, &'static str) = (1e141, "sexquadragintillion");
    pub const SEPTENQUADRAGINTILLION: (f64, &'static str) = (1e144, "septenquadragintillion");
    pub const OCTOQUADRAGINTILLION: (f64, &'static str) = (1e147, "octoquadragintillion");
    pub const NOVEMQUADRAGINTILLION: (f64, &'static str) = (1e150, "novemquadragintillion");
    pub const QUINQUAGINTILLION: (f64, &'static str) = (1e153, "quinquagintillion");
    pub const UNQUINQUAGINTILLION: (f64, &'static str) = (1e156, "unquinquagintillion");
    pub const DUOQUINQUAGINTILLION: (f64, &'static str) = (1e159, "duoquinquagintillion");
    pub const TREQUINQUAGINTILLION: (f64, &'static str) = (1e162, "trequinquagintillion");
    pub const QUATTUORQUINQUAGINTILLION: (f64, &'static str) = (1e165, "quattuorquinquagintillion");
    pub const QUINQUINQUAGINTILLION: (f64, &'static str) = (1e168, "quinquinquagintillion");
    pub const SEXQUINQUAGINTILLION: (f64, &'static str) = (1e171, "sexquinquagintillion");
    pub const SEPTENQUINQUAGINTILLION: (f64, &'static str) = (1e174, "septenquinquagintillion");
    pub const OCTOQUINQUAGINTILLION: (f64, &'static str) = (1e177, "octoquinquagintillion");
    pub const NOVEMQUINQUAGINTILLION: (f64, &'static str) = (1e180, "novemquinquagintillion");
    pub const SEXAGINTILLION: (f64, &'static str) = (1e183, "sexagintillion");
    pub const UNSEXAGINTILLION: (f64, &'static str) = (1e186, "unsexagintillion");
    pub const DUOSEXAGINTILLION: (f64, &'static str) = (1e189, "duosexagintillion");
    pub const TRESEXAGINTILLION: (f64, &'static str) = (1e192, "tresexagintillion");
    pub const QUATTUORSEXAGINTILLION: (f64, &'static str) = (1e195, "quattuorsexagintillion");
    pub const QUINSEXAGINTILLION: (f64, &'static str) = (1e198, "quinsexagintillion");
    pub const SEXSEXAGINTILLION: (f64, &'static str) = (1e201, "sexsexagintillion");
    pub const SEPTENSEXAGINTILLION: (f64, &'static str) = (1e204, "septensexagintillion");
    pub const OCTOSEXAGINTILLION: (f64, &'static str) = (1e207, "octosexagintillion");
    pub const NOVEMSEXAGINTILLION: (f64, &'static str) = (1e210, "novemsexagintillion");
    pub const SEPTUAGINTILLION: (f64, &'static str) = (1e213, "septuagintillion");
    pub const UNSEPTUAGINTILLION: (f64, &'static str) = (1e216, "unseptuagintillion");
    pub const DUOSEPTUAGINTILLION: (f64, &'static str) = (1e219, "duoseptuagintillion");
    pub const TRESEPTUAGINTILLION: (f64, &'static str) = (1e222, "treseptuagintillion");
    pub const QUATTUORSEPTUAGINTILLION: (f64, &'static str) = (1e225, "quattuorseptuagintillion");
    pub const QUINSEPTUAGINTILLION: (f64, &'static str) = (1e228, "quinseptuagintillion");
    pub const SEXSEPTUAGINTILLION: (f64, &'static str) = (1e231, "sexseptuagintillion");
    pub const SEPTENSEPTUAGINTILLION: (f64, &'static str) = (1e234, "septenseptuagintillion");
    pub const OCTOSEPTUAGINTILLION: (f64, &'static str) = (1e237, "octoseptuagintillion");
    pub const NOVEMSEPTUAGINTILLION: (f64, &'static str) = (1e240, "novemseptuagintillion");
    pub const OCTOGINTILLION: (f64, &'static str) = (1e243, "octogintillion");
    pub const UNOCTOGINTILLION: (f64, &'static str) = (1e246, "unoctogintillion");
    pub const DUOOCTOGINTILLION: (f64, &'static str) = (1e249, "duooctogintillion");
    pub const TREOCTOGINTILLION: (f64, &'static str) = (1e252, "treoctogintillion");
    pub const QUATTUOROCTOGINTILLION: (f64, &'static str) = (1e255, "quattuoroctogintillion");
    pub const QUINOCTOGINTILLION: (f64, &'static str) = (1e258, "quinoctogintillion");
    pub const SEXOCTOGINTILLION: (f64, &'static str) = (1e261, "sexoctogintillion");
    pub const SEPTENOCTOGINTILLION: (f64, &'static str) = (1e264, "septenoctogintillion");
    pub const OCTOOCTOGINTILLION: (f64, &'static str) = (1e267, "octooctogintillion");
    pub const NOVEMOCTOGINTILLION: (f64, &'static str) = (1e270, "novemoctogintillion");
    pub const NONAGINTILLION: (f64, &'static str) = (1e273, "nonagintillion");
    pub const UNNONAGINTILLION: (f64, &'static str) = (1e276, "unnonagintillion");
    pub const DUONONAGINTILLION: (f64, &'static str) = (1e279, "duononagintillion");
    pub const TRENONAGINTILLION: (f64, &'static str) = (1e282, "trenonagintillion");
    pub const QUATTUORNONAGINTILLION: (f64, &'static str) = (1e285, "quattuornonagintillion");
    pub const QUINNONAGINTILLION: (f64, &'static str) = (1e288, "quinnonagintillion");
    pub const SEXNONAGINTILLION: (f64, &'static str) = (1e291, "sexnonagintillion");
    pub const SEPTENNONAGINTILLION: (f64, &'static str) = (1e294, "septennonagintillion");
    pub const OCTONONAGINTILLION: (f64, &'static str) = (1e297, "octononagintillion");

    pub fn new(value: f64) -> Self {
        Self {
            inner: value,
            should_use_thousand: true,
        }
    }

    pub fn new_with_thousand(value: f64, should_use_thousand: bool) -> Self {
        Self {
            inner: value,
            should_use_thousand,
        }
    }

    pub fn thousand(&self) -> f64 {
        self.inner * Self::THOUSAND.0
    }
    pub fn million(&self) -> f64 {
        self.inner * Self::MILLION.0
    }
    pub fn billion(&self) -> f64 {
        self.inner * Self::BILLION.0
    }
    pub fn trillion(&self) -> f64 {
        self.inner * Self::TRILLION.0
    }
    pub fn quadrillion(&self) -> f64 {
        self.inner * Self::QUADRILLION.0
    }
    pub fn quintillion(&self) -> f64 {
        self.inner * Self::QUINTILLION.0
    }
    pub fn sextillion(&self) -> f64 {
        self.inner * Self::SEXTILLION.0
    }
    pub fn septillion(&self) -> f64 {
        self.inner * Self::SEPTILLION.0
    }
    pub fn octillion(&self) -> f64 {
        self.inner * Self::OCTILLION.0
    }
    pub fn nonillion(&self) -> f64 {
        self.inner * Self::NONILLION.0
    }
    pub fn decillion(&self) -> f64 {
        self.inner * Self::DECILLION.0
    }
    pub fn undecillion(&self) -> f64 {
        self.inner * Self::UNDECILLION.0
    }
    pub fn duodecillion(&self) -> f64 {
        self.inner * Self::DUODECILLION.0
    }
    pub fn tredecillion(&self) -> f64 {
        self.inner * Self::TREDECILLION.0
    }
    pub fn quattuordecillion(&self) -> f64 {
        self.inner * Self::QUATTUORDECILLION.0
    }
    pub fn quindecillion(&self) -> f64 {
        self.inner * Self::QUINDECILLION.0
    }
    pub fn sexdecillion(&self) -> f64 {
        self.inner * Self::SEXDECILLION.0
    }
    pub fn septendecillion(&self) -> f64 {
        self.inner * Self::SEPTENDECILLION.0
    }
    pub fn octodecillion(&self) -> f64 {
        self.inner * Self::OCTODECILLION.0
    }
    pub fn novemdecillion(&self) -> f64 {
        self.inner * Self::NOVEMDECILLION.0
    }
    pub fn vigintillion(&self) -> f64 {
        self.inner * Self::VIGINTILLION.0
    }
    pub fn unvigintillion(&self) -> f64 {
        self.inner * Self::UNVIGINTILLION.0
    }
    pub fn duovigintillion(&self) -> f64 {
        self.inner * Self::DUOVIGINTILLION.0
    }
    pub fn trevigintillion(&self) -> f64 {
        self.inner * Self::TREVIGINTILLION.0
    }
    pub fn quattuorvigintillion(&self) -> f64 {
        self.inner * Self::QUATTUORVIGINTILLION.0
    }
    pub fn quinvigintillion(&self) -> f64 {
        self.inner * Self::QUINVIGINTILLION.0
    }
    pub fn sexvigintillion(&self) -> f64 {
        self.inner * Self::SEXVIGINTILLION.0
    }
    pub fn septenvigintillion(&self) -> f64 {
        self.inner * Self::SEPTENVIGINTILLION.0
    }
    pub fn octovigintillion(&self) -> f64 {
        self.inner * Self::OCTOVIGINTILLION.0
    }
    pub fn novemvigintillion(&self) -> f64 {
        self.inner * Self::NOVEMVIGINTILLION.0
    }
    pub fn trigintillion(&self) -> f64 {
        self.inner * Self::TRIGINTILLION.0
    }
    pub fn untrigintillion(&self) -> f64 {
        self.inner * Self::UNTRIGINTILLION.0
    }
    pub fn duotrigintillion(&self) -> f64 {
        self.inner * Self::DUOTRIGINTILLION.0
    }
    pub fn tretrigintillion(&self) -> f64 {
        self.inner * Self::TRETRIGINTILLION.0
    }
    pub fn quattuortrigintillion(&self) -> f64 {
        self.inner * Self::QUATTUORTRIGINTILLION.0
    }
    pub fn quintrigintillion(&self) -> f64 {
        self.inner * Self::QUINTRIGINTILLION.0
    }
    pub fn sextrigintillion(&self) -> f64 {
        self.inner * Self::SEXTRIGINTILLION.0
    }
    pub fn septentrigintillion(&self) -> f64 {
        self.inner * Self::SEPTENTRIGINTILLION.0
    }
    pub fn octotrigintillion(&self) -> f64 {
        self.inner * Self::OCTOTRIGINTILLION.0
    }
    pub fn novemtrigintillion(&self) -> f64 {
        self.inner * Self::NOVEMTRIGINTILLION.0
    }
    pub fn quadragintillion(&self) -> f64 {
        self.inner * Self::QUADRAGINTILLION.0
    }
    pub fn unquadragintillion(&self) -> f64 {
        self.inner * Self::UNQUADRAGINTILLION.0
    }
    pub fn duoquadragintillion(&self) -> f64 {
        self.inner * Self::DUOQUADRAGINTILLION.0
    }
    pub fn trequadragintillion(&self) -> f64 {
        self.inner * Self::TREQUADRAGINTILLION.0
    }
    pub fn quattuorquadragintillion(&self) -> f64 {
        self.inner * Self::QUATTUORQUADRAGINTILLION.0
    }
    pub fn quinquadragintillion(&self) -> f64 {
        self.inner * Self::QUINQUADRAGINTILLION.0
    }
    pub fn sexquadragintillion(&self) -> f64 {
        self.inner * Self::SEXQUADRAGINTILLION.0
    }
    pub fn septenquadragintillion(&self) -> f64 {
        self.inner * Self::SEPTENQUADRAGINTILLION.0
    }
    pub fn octoquadragintillion(&self) -> f64 {
        self.inner * Self::OCTOQUADRAGINTILLION.0
    }
    pub fn novemquadragintillion(&self) -> f64 {
        self.inner * Self::NOVEMQUADRAGINTILLION.0
    }
    pub fn quinquagintillion(&self) -> f64 {
        self.inner * Self::QUINQUAGINTILLION.0
    }
    pub fn unquinquagintillion(&self) -> f64 {
        self.inner * Self::UNQUINQUAGINTILLION.0
    }
    pub fn duoquinquagintillion(&self) -> f64 {
        self.inner * Self::DUOQUINQUAGINTILLION.0
    }
    pub fn trequinquagintillion(&self) -> f64 {
        self.inner * Self::TREQUINQUAGINTILLION.0
    }
    pub fn quattuorquinquagintillion(&self) -> f64 {
        self.inner * Self::QUATTUORQUINQUAGINTILLION.0
    }
    pub fn quinquinquagintillion(&self) -> f64 {
        self.inner * Self::QUINQUINQUAGINTILLION.0
    }
    pub fn sexquinquagintillion(&self) -> f64 {
        self.inner * Self::SEXQUINQUAGINTILLION.0
    }
    pub fn septenquinquagintillion(&self) -> f64 {
        self.inner * Self::SEPTENQUINQUAGINTILLION.0
    }
    pub fn octoquinquagintillion(&self) -> f64 {
        self.inner * Self::OCTOQUINQUAGINTILLION.0
    }
    pub fn novemquinquagintillion(&self) -> f64 {
        self.inner * Self::NOVEMQUINQUAGINTILLION.0
    }
    pub fn sexagintillion(&self) -> f64 {
        self.inner * Self::SEXAGINTILLION.0
    }
    pub fn unsexagintillion(&self) -> f64 {
        self.inner * Self::UNSEXAGINTILLION.0
    }
    pub fn duosexagintillion(&self) -> f64 {
        self.inner * Self::DUOSEXAGINTILLION.0
    }
    pub fn tresexagintillion(&self) -> f64 {
        self.inner * Self::TRESEXAGINTILLION.0
    }
    pub fn quattuorsexagintillion(&self) -> f64 {
        self.inner * Self::QUATTUORSEXAGINTILLION.0
    }
    pub fn quinsexagintillion(&self) -> f64 {
        self.inner * Self::QUINSEXAGINTILLION.0
    }
    pub fn sexsexagintillion(&self) -> f64 {
        self.inner * Self::SEXSEXAGINTILLION.0
    }
    pub fn septensexagintillion(&self) -> f64 {
        self.inner * Self::SEPTENSEXAGINTILLION.0
    }
    pub fn octosexagintillion(&self) -> f64 {
        self.inner * Self::OCTOSEXAGINTILLION.0
    }
    pub fn novemsexagintillion(&self) -> f64 {
        self.inner * Self::NOVEMSEXAGINTILLION.0
    }
    pub fn septuagintillion(&self) -> f64 {
        self.inner * Self::SEPTUAGINTILLION.0
    }
    pub fn unseptuagintillion(&self) -> f64 {
        self.inner * Self::UNSEPTUAGINTILLION.0
    }
    pub fn duoseptuagintillion(&self) -> f64 {
        self.inner * Self::DUOSEPTUAGINTILLION.0
    }
    pub fn treseptuagintillion(&self) -> f64 {
        self.inner * Self::TRESEPTUAGINTILLION.0
    }
    pub fn quattuorseptuagintillion(&self) -> f64 {
        self.inner * Self::QUATTUORSEPTUAGINTILLION.0
    }
    pub fn quinseptuagintillion(&self) -> f64 {
        self.inner * Self::QUINSEPTUAGINTILLION.0
    }
    pub fn sexseptuagintillion(&self) -> f64 {
        self.inner * Self::SEXSEPTUAGINTILLION.0
    }
    pub fn septenseptuagintillion(&self) -> f64 {
        self.inner * Self::SEPTENSEPTUAGINTILLION.0
    }
    pub fn octoseptuagintillion(&self) -> f64 {
        self.inner * Self::OCTOSEPTUAGINTILLION.0
    }
    pub fn novemseptuagintillion(&self) -> f64 {
        self.inner * Self::NOVEMSEPTUAGINTILLION.0
    }
    pub fn octogintillion(&self) -> f64 {
        self.inner * Self::OCTOGINTILLION.0
    }
    pub fn unoctogintillion(&self) -> f64 {
        self.inner * Self::UNOCTOGINTILLION.0
    }
    pub fn duooctogintillion(&self) -> f64 {
        self.inner * Self::DUOOCTOGINTILLION.0
    }
    pub fn treoctogintillion(&self) -> f64 {
        self.inner * Self::TREOCTOGINTILLION.0
    }
    pub fn quattuoroctogintillion(&self) -> f64 {
        self.inner * Self::QUATTUOROCTOGINTILLION.0
    }
    pub fn quinoctogintillion(&self) -> f64 {
        self.inner * Self::QUINOCTOGINTILLION.0
    }
    pub fn sexoctogintillion(&self) -> f64 {
        self.inner * Self::SEXOCTOGINTILLION.0
    }
    pub fn septenoctogintillion(&self) -> f64 {
        self.inner * Self::SEPTENOCTOGINTILLION.0
    }
    pub fn octooctogintillion(&self) -> f64 {
        self.inner * Self::OCTOOCTOGINTILLION.0
    }
    pub fn novemoctogintillion(&self) -> f64 {
        self.inner * Self::NOVEMOCTOGINTILLION.0
    }
    pub fn nonagintillion(&self) -> f64 {
        self.inner * Self::NONAGINTILLION.0
    }
    pub fn unnonagintillion(&self) -> f64 {
        self.inner * Self::UNNONAGINTILLION.0
    }
    pub fn duononagintillion(&self) -> f64 {
        self.inner * Self::DUONONAGINTILLION.0
    }
    pub fn trenonagintillion(&self) -> f64 {
        self.inner * Self::TRENONAGINTILLION.0
    }
    pub fn quattuornonagintillion(&self) -> f64 {
        self.inner * Self::QUATTUORNONAGINTILLION.0
    }
    pub fn quinnonagintillion(&self) -> f64 {
        self.inner * Self::QUINNONAGINTILLION.0
    }
    pub fn sexnonagintillion(&self) -> f64 {
        self.inner * Self::SEXNONAGINTILLION.0
    }
    pub fn septennonagintillion(&self) -> f64 {
        self.inner * Self::SEPTENNONAGINTILLION.0
    }
    pub fn octononagintillion(&self) -> f64 {
        self.inner * Self::OCTONONAGINTILLION.0
    }
}

impl From<LingNum> for String {
    fn from(value: LingNum) -> Self {
        let mut units = vec![
            LingNum::OCTONONAGINTILLION,
            LingNum::SEPTENNONAGINTILLION,
            LingNum::SEXNONAGINTILLION,
            LingNum::QUINNONAGINTILLION,
            LingNum::QUATTUORNONAGINTILLION,
            LingNum::TRENONAGINTILLION,
            LingNum::DUONONAGINTILLION,
            LingNum::UNNONAGINTILLION,
            LingNum::NONAGINTILLION,
            LingNum::NOVEMOCTOGINTILLION,
            LingNum::OCTOOCTOGINTILLION,
            LingNum::SEPTENOCTOGINTILLION,
            LingNum::SEXOCTOGINTILLION,
            LingNum::QUINOCTOGINTILLION,
            LingNum::QUATTUOROCTOGINTILLION,
            LingNum::TREOCTOGINTILLION,
            LingNum::DUOOCTOGINTILLION,
            LingNum::UNOCTOGINTILLION,
            LingNum::OCTOGINTILLION,
            LingNum::NOVEMSEPTUAGINTILLION,
            LingNum::OCTOSEPTUAGINTILLION,
            LingNum::SEPTENSEPTUAGINTILLION,
            LingNum::SEXSEPTUAGINTILLION,
            LingNum::QUINSEPTUAGINTILLION,
            LingNum::QUATTUORSEPTUAGINTILLION,
            LingNum::TRESEPTUAGINTILLION,
            LingNum::DUOSEPTUAGINTILLION,
            LingNum::UNSEPTUAGINTILLION,
            LingNum::SEPTUAGINTILLION,
            LingNum::NOVEMSEXAGINTILLION,
            LingNum::OCTOSEXAGINTILLION,
            LingNum::SEPTENSEXAGINTILLION,
            LingNum::SEXSEXAGINTILLION,
            LingNum::QUINSEXAGINTILLION,
            LingNum::QUATTUORSEXAGINTILLION,
            LingNum::TRESEXAGINTILLION,
            LingNum::DUOSEXAGINTILLION,
            LingNum::UNSEXAGINTILLION,
            LingNum::SEXAGINTILLION,
            LingNum::NOVEMQUINQUAGINTILLION,
            LingNum::OCTOQUINQUAGINTILLION,
            LingNum::SEPTENQUINQUAGINTILLION,
            LingNum::SEXQUINQUAGINTILLION,
            LingNum::QUINQUINQUAGINTILLION,
            LingNum::QUATTUORQUINQUAGINTILLION,
            LingNum::TREQUINQUAGINTILLION,
            LingNum::DUOQUINQUAGINTILLION,
            LingNum::UNQUINQUAGINTILLION,
            LingNum::QUINQUAGINTILLION,
            LingNum::NOVEMQUADRAGINTILLION,
            LingNum::OCTOQUADRAGINTILLION,
            LingNum::SEPTENQUADRAGINTILLION,
            LingNum::SEXQUADRAGINTILLION,
            LingNum::QUINQUADRAGINTILLION,
            LingNum::QUATTUORQUADRAGINTILLION,
            LingNum::TREQUADRAGINTILLION,
            LingNum::DUOQUADRAGINTILLION,
            LingNum::UNQUADRAGINTILLION,
            LingNum::QUADRAGINTILLION,
            LingNum::NOVEMTRIGINTILLION,
            LingNum::OCTOTRIGINTILLION,
            LingNum::SEPTENTRIGINTILLION,
            LingNum::SEXTRIGINTILLION,
            LingNum::QUINTRIGINTILLION,
            LingNum::QUATTUORTRIGINTILLION,
            LingNum::TRETRIGINTILLION,
            LingNum::DUOTRIGINTILLION,
            LingNum::UNTRIGINTILLION,
            LingNum::TRIGINTILLION,
            LingNum::NOVEMVIGINTILLION,
            LingNum::OCTOVIGINTILLION,
            LingNum::SEPTENVIGINTILLION,
            LingNum::SEXVIGINTILLION,
            LingNum::QUINVIGINTILLION,
            LingNum::QUATTUORVIGINTILLION,
            LingNum::TREVIGINTILLION,
            LingNum::DUOVIGINTILLION,
            LingNum::UNVIGINTILLION,
            LingNum::VIGINTILLION,
            LingNum::NOVEMDECILLION,
            LingNum::OCTODECILLION,
            LingNum::SEPTENDECILLION,
            LingNum::SEXDECILLION,
            LingNum::QUINDECILLION,
            LingNum::QUATTUORDECILLION,
            LingNum::TREDECILLION,
            LingNum::DUODECILLION,
            LingNum::UNDECILLION,
            LingNum::DECILLION,
            LingNum::NONILLION,
            LingNum::OCTILLION,
            LingNum::SEPTILLION,
            LingNum::SEXTILLION,
            LingNum::QUINTILLION,
            LingNum::QUADRILLION,
            LingNum::TRILLION,
            LingNum::BILLION,
            LingNum::MILLION,
        ];

        if value.should_use_thousand {
            units.push(LingNum::THOUSAND);
        }

        let abs_value = value.inner.abs();

        if abs_value < 1000.0 {
            return value.inner.to_string();
        }

        for (unit_value, unit_name) in units.iter() {
            if abs_value >= *unit_value {
                let result = value.inner / unit_value;
                return format!("{} {}", result, unit_name);
            }
        }

        value.inner.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_default() {
        let num = LingNum::new(1000.0);
        assert_eq!(num.inner, 1000.0);
        assert!(num.should_use_thousand);
    }

    #[test]
    fn test_new_with_thousand() {
        let num1 = LingNum::new_with_thousand(1000.0, true);
        assert!(num1.should_use_thousand);

        let num2 = LingNum::new_with_thousand(1000.0, false);
        assert!(!num2.should_use_thousand);
    }

    #[test]
    fn test_to_string_small_numbers() {
        let num = LingNum::new(500.0);
        let result: String = num.into();
        assert_eq!(result, "500");
    }

    #[test]
    fn test_to_string_thousand() {
        let num = LingNum::new(1500.0);
        let result: String = num.into();
        assert_eq!(result, "1.5 thousand");
    }

    #[test]
    fn test_to_string_million() {
        let num = LingNum::new(2500000.0);
        let result: String = num.into();
        assert_eq!(result, "2.5 million");
    }

    #[test]
    fn test_to_string_billion() {
        let num = LingNum::new(3750000000.0);
        let result: String = num.into();
        assert_eq!(result, "3.75 billion");
    }

    #[test]
    fn test_to_string_trillion() {
        let num = LingNum::new(1e13);
        let result: String = num.into();
        assert_eq!(result, "10 trillion");
    }

    #[test]
    fn test_to_string_without_thousand() {
        let num = LingNum::new_with_thousand(1500.0, false);
        let result: String = num.into();
        assert_eq!(result, "1500");
    }

    #[test]
    fn test_to_string_without_thousand_large() {
        let num = LingNum::new_with_thousand(999999.0, false);
        let result: String = num.into();
        assert_eq!(result, "999999");
    }

    #[test]
    fn test_to_string_without_thousand_million() {
        let num = LingNum::new_with_thousand(2500000.0, false);
        let result: String = num.into();
        assert_eq!(result, "2.5 million");
    }

    #[test]
    fn test_multiplication_methods() {
        let num = LingNum::new(2.0);
        assert_eq!(num.thousand(), 2000.0);
        assert_eq!(num.million(), 2000000.0);
        assert_eq!(num.billion(), 2000000000.0);
        assert_eq!(num.trillion(), 2000000000000.0);
    }

    #[test]
    fn test_negative_numbers() {
        let num = LingNum::new(-1500.0);
        let result: String = num.into();
        assert_eq!(result, "-1.5 thousand");
    }

    #[test]
    fn test_zero() {
        let num = LingNum::new(0.0);
        let result: String = num.into();
        assert_eq!(result, "0");
    }

    #[test]
    fn test_large_units() {
        let num = LingNum::new(1e100);
        let result: String = num.into();
        assert_eq!(result, "10 duotrigintillion");
    }

    #[test]
    fn test_very_large_units() {
        let num = LingNum::new(1e297);
        let result: String = num.into();
        assert_eq!(result, "1 octononagintillion");
    }
}
