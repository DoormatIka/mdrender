pub struct Documents {
    pub children: Vec<Block>,
}

pub enum ListKind {
    Bullet,
    Ordered,
}
pub struct RawListData {
    pub kind: ListKind,
    pub loose: bool,
    pub items: Vec<RawBlock>,
}
pub struct ListData {
    pub kind: ListKind,
    pub loose: bool,
    pub items: Vec<Block>,
}

pub enum RawBlock {
    BlockQuote(Vec<RawBlock>),
    List(RawListData),
    ListItem(Vec<RawBlock>),

    Paragraph(Vec<String>),
    Heading {
        level: u8,
        raw: String,
    },

    ThematicBreak,
    CodeBlock {
        info: Option<String>,
        fenced: bool,
        literal: String,
    },
    HtmlBlock(String),
}

pub enum Block {
    BlockQuote(Vec<Block>),
    List(ListData),
    ListItem(Vec<Block>),

    Paragraph(Vec<Inline>),
    Heading {
        level: u8,
        content: Vec<Inline>,
    },

    ThematicBreak,
    CodeBlock {
        info: Option<String>,
        fenced: bool,
        literal: String,
    },
    HtmlBlock(String),
}

pub struct LinkContent {
    pub text: Option<String>,
    pub title: Option<String>,
    pub destination: Option<String>,
}
pub struct AutolinkContent {
    pub destination: String,
}

pub enum Inline {
    Codespan(String),
    Emph(String),
    Strong(String),
    Link(LinkContent),
    Image(LinkContent),
    Autolink(AutolinkContent),
    RawHTML(String),
    HardBreak,
    SoftBreak,
    Text(String),
}
