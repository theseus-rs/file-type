use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_149: FileType = FileType {
    file_format: &FileFormat {
        id: 149,
        source_type: SourceType::Linguist,
        name: "HTML+EEX",
        extensions: &["heex", "html.eex", "leex"],
        media_types: &["text/html"],
        signatures: &[],
        related_formats: &[],
    },
};
