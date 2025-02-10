use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_156: FileType = FileType {
    file_format: &FileFormat {
        id: 156,
        source_type: SourceType::Linguist,
        name: "Harbour",
        extensions: &["hb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
