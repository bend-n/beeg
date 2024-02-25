use std::process::ExitCode;

static TAB: phf::Map<&str, u16> = phf::phf_map! {
    "m" => 6,
    "b" => 9,
    "tr" => 12,
    "quadr" => 15,
    "quint" => 18,
    "sext" => 21,
    "sept" => 24,
    "oct" => 27,
    "non" => 30,
    "dec" => 33,
    "undec" => 36,
    "duodec" => 39,
    "tredec" => 42,
    "quattuordec" => 45,
    "quindec" => 48,
    "sexdec" => 51,
    "septendec" => 54,
    "octodec" => 57,
    "novemdec" => 60,
    "vigint" => 63,
    "cent" => 303,
};

fn main() -> ExitCode {
    let re = regex::Regex::new(r"([0-9]+(?:\.[0-9]+)?)[^\S]*(m|b|tr|quadr|quint|sext|sept|oct|non|dec|undec|duodec|tredec|quattuordec|quindec|sexdec|septendec|octodecc|novemdec|vigint|cent)illion").unwrap();
    let Some(x) = std::env::args().skip(1).reduce(|acc, x| acc + &x) else {
        comat::cprintln!("{red}require argument{reset}");
        return ExitCode::FAILURE;
    };

    let Some(mat) = re.captures(&x) else {
        comat::cprintln!("{red}wher num?{reset}");
        return ExitCode::SUCCESS;
    };
    let exponent = TAB[mat.get(2).unwrap().into()];
    println!("{}e+{exponent}", mat.get(1).unwrap().as_str());
    clipp::copy(format!("{}e+{exponent}", mat.get(1).unwrap().as_str()));
    ExitCode::SUCCESS
}
