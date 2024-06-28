use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize)]
struct Paragraph {
    name: String,
}

#[derive(Serialize, Deserialize)]
struct Article {
    article: String,
    author: String,
    paragraph: Vec<Paragraph>,
}

fn read_json_typed(raw_json: &str) -> Article {
    let parsed: Article = serde_json::from_str(raw_json).unwrap();
    return parsed;
}
fn main() {
    let json = r#"
    {
        "article": "How to work with JSON in Rust",
        "author": "Erik Hedman",
        "paragraph": [
            {
                "name": "starting sentence"
            },
            {
                "name": "body of paragraph"
            },
            {
                "name": "end of the paragraph"
            }
        ]
    }"#;

    let parsed: Article = read_json_typed(json);

    for line in &parsed.paragraph {
        println!("\n\nThe name of this line is : {}", line.name)
    }

    println!(
        "\n\n The name of the first paragraph is : {}",
        parsed.paragraph[0].name
    );
}
