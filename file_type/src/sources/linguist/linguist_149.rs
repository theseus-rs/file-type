use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_149: FileFormat = FileFormat {
    id: 149,
    source_type: SourceType::Linguist,
    name: "HTML+EEX",
    extensions: &["heex", "html.eex", "leex"],
    media_types: &["text/html"],
    internal_signatures: &[],
    related_formats: &[],
};
