pub enum Genngo {
    Heisei(u32),  // 1989 - 2019
    Reiwa(u32),   // 2019 - ima
}

impl Genngo {
    fn get_ce_year(&self) -> u32 {
        match self {
            Genngo::Heisei(n) => 1988 + n,
            Genngo::Reiwa(n) => 2018 + n,
        }
    }
}

/// C.E.
/// dosen't provide before MeiJi
pub struct Date {
    year: u32,
    month: u32,
    day: u32,
}

impl Date {
    /// from 平成31/4/29
    pub fn from_japan_locale(locale: String) -> Option<Date> {
        if locale.len() < 10 {
            return None;
        }
        if let Some(genngo) = locale.get(..6) {
            let date = locale.get(6..).unwrap();
            let spines: Vec<u32> = date
                .split('/')
                .map(|s| u32::from_str_radix(s, 10).unwrap())
                .collect();
            assert_eq!(spines.len(), 3, "date invalid");
            let genngo = match genngo {
                "平成" => Genngo::Heisei(spines[0]),
                "令和" => Genngo::Reiwa(spines[0]),
                _ => unreachable!(),
            };
            Some(Date {
                year: genngo.get_ce_year(),
                month: spines[1],
                day: spines[2],
            })
        } else {
            None
        }
    }

    pub fn to_iso_string(&self) -> String {
        format!(
            "{:04}-{:02}-{:02}T00:00:00Z",
            self.year, self.month, self.day
        )
    }
}

pub fn from_jp_to_iso(jp: String) -> Option<String> {
    Date::from_japan_locale(jp).map(|x| x.to_iso_string())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn from_jp_to_iso() {
        let input = String::from("平成31/4/29");
        let output = Date::from_japan_locale(input).unwrap().to_iso_string();
        assert_eq!(output, "2019-04-29T00:00:00Z".to_string());
    }
    #[test]
    fn reiwa() {
        let input = String::from("令和1/5/1");
        let output = Date::from_japan_locale(input).unwrap().to_iso_string();
        assert_eq!(output, "2019-05-01T00:00:00Z".to_string());
    }
}
