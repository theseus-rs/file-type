use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_239946126: FileType = FileType {
    file_format: &FileFormat {
        id: 239_946_126,
        source_type: SourceType::Linguist,
        name: "Fennel",
        extensions: &["fnl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
