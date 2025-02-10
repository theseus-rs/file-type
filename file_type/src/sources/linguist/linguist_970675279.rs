use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_970675279: FileType = FileType {
    file_format: &FileFormat {
        id: 970_675_279,
        source_type: SourceType::Linguist,
        name: "kvlang",
        extensions: &["kv"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
