use core::fmt;

// the working structure the parser actually mutates.
// indices used instead of addresses to avoid annoying the borrow checker.
pub struct Arena {
    nodes: Vec<OpenBlock>,
}
impl Arena {
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    // inserts a node and returns its index.
    // pushes an index to the child's parent as well.
    pub fn push(&mut self, block: OpenBlock) -> usize {
        let idx = self.nodes.len();
        if let Some(parent_idx) = block.parent {
            self.nodes[parent_idx].children.push(idx);
        }
        self.nodes.push(block);

        idx
    }

    pub fn get(&self, idx: usize) -> Option<&OpenBlock> {
        self.nodes.get(idx)
    }

    pub fn get_mut(&mut self, idx: usize) -> Option<&mut OpenBlock> {
        self.nodes.get_mut(idx)
    }
}

pub struct OpenBlock {
    pub kind: OpenBlockKind,
    pub parent: Option<usize>,
    pub children: Vec<usize>, // only meaningful for container kinds
    pub is_open: bool,
}
impl OpenBlock {
    pub fn new(kind: OpenBlockKind, parent: Option<usize>) -> Self {
        Self {
            kind,
            parent,
            children: Vec::new(),
            is_open: true,
        }
    }
}
impl fmt::Display for OpenBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "OpenBlock {{")?;
        writeln!(f, "   kind: {}", self.kind)?;
        writeln!(f, "   is_open: {}", self.is_open)?;
        writeln!(f, "   parent: {:?}", self.parent)?;
        writeln!(f, "   children: {:?}", self.children)?;
        writeln!(f, "OpenBlock }}")?;

        Ok(())
    }
}

pub enum OpenBlockKind {
    Document,
    BlockQuote,
    List(ListData<RawBlock>),
    ListItem,
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
impl fmt::Display for OpenBlockKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Document => writeln!(f, "[DOCUMENT]"),
            Self::BlockQuote => writeln!(f, "[BLOCK QUOTE]"),
            Self::List(data) => {
                let items: Vec<String> = data.items.iter().map(|v| v.to_string()).collect();
                writeln!(
                    f,
                    "[LIST (kind: {}, loose: {}, items: {:?})]",
                    data.kind, data.loose, items
                )
            }
            _ => todo!(),
        }
    }
}

////// finished phase 1 //////
pub struct Document<T> {
    pub children: Vec<T>,
}
impl<T> Document<T> {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }
}

pub enum ListKind {
    Bullet,
    Ordered,
}
impl fmt::Display for ListKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ListKind::Bullet => writeln!(f, "[BULLET]"),
            ListKind::Ordered => writeln!(f, "[ORDERED]"),
        }
    }
}

pub struct RawListData {
    pub kind: ListKind,
    pub loose: bool,
    pub items: Vec<RawBlock>,
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

impl fmt::Display for RawBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BlockQuote(blocks) => {
                let mut s = String::from("[Block Quote]");
                for block in blocks.iter() {
                    s.push_str(format!("\n  {}", block).as_str());
                }
                f.write_str(s.as_str())
            }
            Self::List(data) => {
                let items: Vec<String> = data.items.iter().map(|v| v.to_string()).collect();
                writeln!(
                    f,
                    "[Raw List (kind: {}, loose: {}, items: {:?})]",
                    data.kind, data.loose, items
                )
            }
            Self::ListItem(items) => {
                let mut s = String::from("[List Item]");
                for block in items.iter() {
                    s.push_str(format!("\n  {}", block).as_str());
                }
                f.write_str(s.as_str())
            }
            Self::Paragraph(s) => {
                let s = s.join("\n");
                writeln!(f, "{}", s)?;

                Ok(())
            }
            _ => todo!(),
        }
    }
}

///////////// finished phase 2 ////////////////

pub enum Block {
    BlockQuote(Vec<Block>),
    List(ListData<Block>),
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

pub struct ListData<T> {
    pub kind: ListKind,
    pub loose: bool,
    pub items: Vec<T>,
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
