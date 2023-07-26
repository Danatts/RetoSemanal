// Dada una URL con parámetros, crea una función que obtenga sus valores.
// No se pueden usar operaciones del lenguaje que realicen esta tarea directamente.

use std::collections::HashMap;

pub fn get_params_from_url(url: &str) -> Result<HashMap<&str, &str>, &str> {
    if url.trim().is_empty() {
        return Err("No URL provided");
    }

    let params: Vec<&str> = url.split('?').collect();

    if params.len() == 1 {
        return Err("No params provided");
    }

    let mut result: HashMap<&str, &str> = HashMap::new();

    let params: Vec<&str> = params[1].split('&').collect();

    for param in params {
        let values: Vec<&str> = param.split('=').collect();
        result.insert(values[0], values[1]);
    }
    Ok(result)
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn get_one_param() {
        let params = get_params_from_url("https://retosdeprogramacion.com?year=2023").unwrap();
        assert_eq!(params, HashMap::from([("year", "2023")]));
    }

    #[test]
    fn get_two_params() {
        let params =
            get_params_from_url("https://retosdeprogramacion.com?year=2023&challenge=0").unwrap();
        assert_eq!(
            params,
            HashMap::from([("year", "2023"), ("challenge", "0")])
        );
    }

    #[test]
    fn get_three_params() {
        let params = get_params_from_url(
            "https://retosdeprogramacion.com?year=2023&challenge=0&name=Daniel",
        )
        .unwrap();
        assert_eq!(
            params,
            HashMap::from([("year", "2023"), ("challenge", "0"), ("name", "Daniel")])
        );
    }

    #[test]
    fn get_error_no_params() {
        let params = get_params_from_url("https://retosdeprogramacion.com");
        assert_eq!(params, Err("No params provided"));
    }

    #[test]
    fn get_error_no_url() {
        let params = get_params_from_url("");
        assert_eq!(params, Err("No URL provided"));
    }
}
