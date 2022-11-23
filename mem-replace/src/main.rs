use std::mem;

#[allow(unused)]
#[derive(Debug)]
enum Transformer {
    Vehicle { model: String, sparks: usize },
    Humanoid { name: String, sparks: usize },
}

fn transform(transformer: &mut Transformer, identifier: String) {
    match transformer {
        Transformer::Vehicle { model, sparks } => {
            println!("-- {model} into: {}", identifier);

            *transformer = Transformer::Humanoid {
                name: identifier,
                sparks: mem::take(sparks),
            };
        }
        Transformer::Humanoid { name, sparks } => {
            println!("-- {name} into: {}", identifier);

            *transformer = Transformer::Vehicle {
                model: identifier,
                sparks: mem::take(sparks),
            }
        }
    }
}

fn main() {
    let mut optimus_prime = Transformer::Vehicle {
        model: "Peterbilt Model 379".to_owned(),
        sparks: 1_000,
    };

    println!("{optimus_prime:?}");

    transform(&mut optimus_prime, "Optimus Prime".to_owned());

    println!("{optimus_prime:?}");
}
