use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_399230729: FileType = FileType {
    file_format: &FileFormat {
        id: 399_230_729,
        source_type: SourceType::Linguist,
        name: "VBA",
        extensions: &["bas", "cls", "frm", "vba"],
        media_types: &["text/x-vb"],
        signatures: &[],
        related_formats: &[],
    },
};
