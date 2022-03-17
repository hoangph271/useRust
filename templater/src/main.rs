use anyhow::Result;
use handlebars::Handlebars;
use serde::Serialize;

fn main() -> Result<()> {
    let mut handlebars = Handlebars::new();
    let template = "Hello, {{ name }}...!";

    #[derive(Serialize)]
    struct Person {
        name: String,
    }

    if handlebars
        .register_template_string("hello_message", &template)
        .is_ok()
    {
        println!(
            "{}",
            handlebars.render(
                "hello_message",
                &Person {
                    name: "@HHP".to_owned()
                }
            )?
        );
    }

    // * Without register_template_string
    let girl = Person {
        name: "#Her".to_owned(),
    };

    if let Ok(output) = handlebars.render_template("There go the girl, she's named {{ name }}...!", &girl)
    {
        println!("{}", output);
    }

    Ok(())
}
