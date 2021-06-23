pub enum Genngo {
    Heisei(u32), // 1989 - 2019
    Reiwa(u32),  // 2019 - ima
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
    hour: u32,
    minute: u32,
    second: u32,
}

impl Date {
    /// from 平成31/4/29
    pub fn from_japan_locale(locale: String) -> anyhow::Result<Date> {
        if let Some(genngo) = locale.get(..6) {
            let date = locale.get(6..).ok_or_else(|| anyhow::Error::msg("parse failed"))?;
            let spines: Result<Vec<u32>, _> = date
                .replace("元年", "1年")
                .split(' ')
                .flat_map(|s| s.split('/'))
                .flat_map(|s| s.split(':'))
                .flat_map(|s| s.split('年'))
                .flat_map(|s| s.split('月'))
                .flat_map(|s| s.split('日'))
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<u32>())
                .collect();
            let mut spines = spines?;
            let mut push = spines.len();
            if push < 6 {
                push = 6 - push;
                spines.append(&mut vec![0; push]);
            }
            let genngo = match genngo {
                "平成" => Genngo::Heisei(spines[0]),
                "令和" => Genngo::Reiwa(spines[0]),
                _ => unreachable!(),
            };
            return Ok(Date {
                year: genngo.get_ce_year(),
                month: spines[1],
                day: spines[2],
                hour: spines[3],
                minute: spines[4],
                second: spines[5],
            });
        }
        Err(anyhow::Error::msg("parse failed"))
    }

    pub fn to_iso_string(&self) -> String {
        format!(
            "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
            self.year, self.month, self.day, self.hour, self.minute, self.second
        )
    }
}

pub fn from_jp_to_iso(jp: String) -> anyhow::Result<String> {
    Date::from_japan_locale(jp).map(|x| x.to_iso_string())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn from_jp_to_iso() {
        let input = String::from("平成31/4/29 23:45");
        let output = Date::from_japan_locale(input).unwrap().to_iso_string();
        assert_eq!(output, "2019-04-29T23:45:00Z".to_string());
    }

    #[test]
    fn from_jp_to_iso2() {
        let input = String::from("平成31年/4/29 23:45");
        let output = Date::from_japan_locale(input).unwrap().to_iso_string();
        assert_eq!(output, "2019-04-29T23:45:00Z".to_string());
    }

    #[test]
    fn from_jp_to_iso3() {
        let input = String::from("平成元年/4/29 23:45");
        let output = Date::from_japan_locale(input).unwrap().to_iso_string();
        assert_eq!(output, "1989-04-29T23:45:00Z".to_string());
    }

    #[test]
    fn from_jp_to_iso4() {
        let input = String::from("令和3年4月1日");
        let output = Date::from_japan_locale(input).unwrap().to_iso_string();
        assert_eq!(output, "2021-04-01T00:00:00Z".to_string());
    }

    #[test]
    fn reiwa() {
        let input = String::from("令和1/5/1");
        let output = Date::from_japan_locale(input).unwrap().to_iso_string();
        assert_eq!(output, "2019-05-01T00:00:00Z".to_string());
    }
}
