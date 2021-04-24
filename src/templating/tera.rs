use ::tera::Tera;

pub fn init(glob: &str) -> Tera {
    let mut tera = match Tera::new(glob) {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };
    tera.autoescape_on(vec![]); // disable autoescaping completely
    //tera.autoescape_on(vec!["html", ".sql"]);
    //tera.register_filter("do_nothing", do_nothing_filter);
    tera
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::indoc::indoc;
    use crate::templating::vars::Vars;

    fn fab_tera() -> Tera {
        let glob = &*format!("{}/templates/**/*", env!("CARGO_MANIFEST_DIR"));
        super::init(glob)
    }

    const FAB_OUTPUT_HTML: &str = indoc!{r#"
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8">
                <title>my title</title>
                <meta name="viewport" content="width=device-width, initial-scale=1">
            </head>
            <body>
        my content
            </body>
        </html>
    "#};

    #[test]
    fn test_tera_x_vars() {
        let tera = fab_tera();
        let vars = Vars {
            title: Some("my title".into()),
            content: Some("my content".into()),
        };
        let actual = tera.render(
            "example.html",
            &::tera::Context::from_serialize(&vars).unwrap()
        ).unwrap();
        assert_eq!(actual, FAB_OUTPUT_HTML);
    }

    #[test]
    fn test_tera_x_json() {
        let tera = fab_tera();
        let vars = indoc!{r#"
            {
                "title": "my title",
                "content": "my content"
            }
        "#};
        let vars: ::serde_json::Value = ::serde_json::from_str(vars).unwrap();
        let actual = tera.render(
            "example.html",
            &::tera::Context::from_serialize(&vars).unwrap()
        ).unwrap();
        assert_eq!(actual, FAB_OUTPUT_HTML);
    }

    #[test]
    fn test_tera_x_toml() {
        let tera = fab_tera();
        let vars = indoc!{r#"
            title = "my title"
            content = "my content"
        "#};
        let vars: ::toml::Value = vars.parse::<::toml::Value>().unwrap();
        let actual = tera.render(
            "example.html",
            &::tera::Context::from_serialize(&vars).unwrap()
        ).unwrap();
        assert_eq!(actual, FAB_OUTPUT_HTML);
    }

    #[test]
    fn test_tera_x_yaml() {
        let tera = fab_tera();
        let vars = indoc!{r#"
            title: "my title"
            content: "my content"
        "#};
        let vars: ::serde_yaml::Value = ::serde_yaml::from_str(&vars).unwrap();
        let actual = tera.render(
            "example.html",
            &::tera::Context::from_serialize(&vars).unwrap()
        ).unwrap();
        assert_eq!(actual, FAB_OUTPUT_HTML);
    }

}
