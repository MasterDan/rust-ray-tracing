use std::io::Error;
use std::io::ErrorKind;
use std::io::Result;

pub(crate) struct AspectRatioString(pub String);

impl AspectRatioString {
    pub fn parse(&self) -> Option<f64> {
        if let Ok(res) = self.try_split("/") {
            return Some(res);
        }
        if let Ok(res) = self.try_split(" ") {
            return Some(res);
        }
        if let Ok(res) = self.try_split("-") {
            return Some(res);
        }
        if let Ok(res) = self.try_split(",") {
            return Some(res);
        }
        None
    }

    fn try_split(&self, splitter: &str) -> Result<f64> {
        let splitted: Vec<&str> = self.0.split(splitter).filter(|el| !el.is_empty()).collect();
        if splitted.len() != 2 {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "Cannot split aspect ratio with '{}' on two parts. Got: {}",
                    splitter,
                    splitted.len()
                ),
            ));
        }

        let width = match splitted[0].trim().parse::<f64>() {
            Ok(val) => val,
            Err(e) => {
                return Err(Error::new(
                    ErrorKind::Other,
                    format!("Cannot parse '{}' because of {}", splitted[0], e),
                ))
            }
        };

        let height = match splitted[1].trim().parse::<f64>() {
            Ok(val) => val,
            Err(e) => {
                return Err(Error::new(
                    ErrorKind::Other,
                    format!("Cannot parse '{}' because of {}", splitted[1], e),
                ))
            }
        };

        Ok(width / height)
    }
}

#[cfg(test)]
mod tests {

    use crate::config::aspect_ratio_string::AspectRatioString;

    #[test]
    pub fn parse() {
        let ars_1 = AspectRatioString(String::from("16 / 9"));
        let ars_2 = AspectRatioString(String::from("16 - 9"));
        let ars_3 = AspectRatioString(String::from("16 , 9"));
        let ars_4 = AspectRatioString(String::from("16  9"));
        let a_r = 16_f64 / 9_f64;
        let spl_res_1 = ars_1.try_split("/").unwrap();
        let spl_res_2 = ars_2.try_split("-").unwrap();
        let spl_res_3 = ars_3.try_split(",").unwrap();
        let spl_res_4 = ars_4.try_split(" ").unwrap();
        assert_eq!(a_r, spl_res_1);
        assert_eq!(a_r, spl_res_2);
        assert_eq!(a_r, spl_res_3);
        assert_eq!(a_r, spl_res_4);
        let a_r_parsed_1 = ars_1.parse().expect("Parsing failed");
        let a_r_parsed_2 = ars_2.parse().expect("Parsing failed");
        let a_r_parsed_3 = ars_3.parse().expect("Parsing failed");
        let a_r_parsed_4 = ars_4.parse().expect("Parsing failed");
        assert_eq!(a_r, a_r_parsed_1);
        assert_eq!(a_r, a_r_parsed_2);
        assert_eq!(a_r, a_r_parsed_3);
        assert_eq!(a_r, a_r_parsed_4);
    }
}
