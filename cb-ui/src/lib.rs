// # Style

struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

// Style groups are collections of style values belonging to the same category.

struct ColorStyleGroup {
    bgColor: Color,
    fgColor: Color,
}

struct FontStyleGroup {
    strikeThrough: Option<bool>,
}

// Styles are collections of style groups applicable to `Node`.

struct ContainerStyle {
    color: Option<ColorStyleGroup>,
}

struct TextStyle {
    color: Option<ColorStyleGroup>,
    font: Option<FontStyleGroup>,
}

// # Layout

enum Sizing {
    Ratio(u8),
    Fit,
    Expand,
}

// Layout overrides for individual nodes
struct ContainerLayout {
    // Exclude from layout
    hidden: Option<bool>,
    // How node calculates its size
    sizing: Option<Sizing>,
}

enum Node {
    // Container with horizontal layout
    HBox {
        style: Option<ContainerStyle>,
        layout: Option<ContainerLayout>,
        children: Vec<Node>,
    },
    // Container with vertical layout
    VBox {
        style: Option<ContainerStyle>,
        layout: Option<ContainerLayout>,
        children: Vec<Node>,
    },
    // Text element
    Text {
        style: Option<TextStyle>,
        layout: Option<ContainerLayout>,
        context: String,
    },
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
