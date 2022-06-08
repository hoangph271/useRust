trait Widget {
    fn draw(&self);
}

pub struct Screen {
    widgets: Vec<Box<dyn Widget>>,
}
impl Widget for Screen {
    fn draw(&self) {
        for widget in &self.widgets {
            widget.draw();
        }

        println!("Render Screen finished...!");
    }
}

pub struct Button {
    text: String,
}
impl Widget for Button {
    fn draw(&self) {
        println!("[Button] - {}", self.text)
    }
}

struct Text {
    value: String,
}
impl Widget for Text {
    fn draw(&self) {
        println!("[Text] - \"{}\"", &self.value)
    }
}
struct Container {
    widgets: Vec<Box<dyn Widget>>,
}
impl Widget for Container {
    fn draw(&self) {
        println!("[Container] -#-");

        for widget in &self.widgets {
            widget.draw()
        }

        println!("[Container] -/-");
    }
}

fn main() {
    let screen = Screen {
        widgets: vec![
            Box::new(Text {
                value: "Default Text value".to_owned(),
            }),
            Box::new(Button {
                text: "OK".to_owned(),
            }),
            Box::new(Container {
                widgets: vec![
                    Box::new(Text {
                        value: "Text in container".to_owned(),
                    }),
                    Box::new(Button {
                        text: "Submit".to_owned(),
                    }),
                ],
            }),
        ],
    };

    screen.draw();
}
