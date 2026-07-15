use std::fmt;

use crate::parser::{blocks::OpenBlockKind, parser::BlockParser};

impl fmt::Display for BlockParser {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_node(f, 0, 0)
    }
}

impl BlockParser {
    fn fmt_node(&self, f: &mut fmt::Formatter<'_>, idx: usize, depth: usize) -> fmt::Result {
        let node = self.arena.get(idx);
        let indent = "  ".repeat(depth);

        if let Some(node) = node {
            let open_marker = if node.is_open { "*" } else { "" };
            match &node.kind {
                OpenBlockKind::Document => {
                    writeln!(f, "{indent}Document{open_marker}")?;
                }
                OpenBlockKind::BlockQuote => {
                    writeln!(f, "{indent}BlockQuote{open_marker}")?;
                }
                OpenBlockKind::List(data) => {
                    writeln!(f, "{indent}List{open_marker} (ordered: {})", data.kind)?;
                }
                OpenBlockKind::ListItem => {
                    writeln!(f, "{indent}ListItem{open_marker}")?;
                }
                OpenBlockKind::Paragraph(lines) => {
                    writeln!(f, "{indent}Paragraph{open_marker} {:?}", lines)?;
                }
                OpenBlockKind::Heading { level, raw } => {
                    writeln!(f, "{indent}Heading{open_marker} (level {level}) {:?}", raw)?;
                }
                OpenBlockKind::ThematicBreak => {
                    writeln!(f, "{indent}ThematicBreak")?;
                }
                OpenBlockKind::CodeBlock {
                    info,
                    fenced,
                    literal,
                } => {
                    writeln!(
                        f,
                        "{indent}CodeBlock{open_marker} (fenced: {fenced}, info: {info:?}) {:?}",
                        literal
                    )?;
                }
                OpenBlockKind::HtmlBlock(s) => {
                    writeln!(f, "{indent}HtmlBlock{open_marker} {:?}", s)?;
                }
            }

            for &child_idx in &node.children {
                self.fmt_node(f, child_idx, depth + 1)?;
            }
        }

        Ok(())
    }
}
