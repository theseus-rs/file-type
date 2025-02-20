use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_29650319: FileType = FileType {
    file_format: &FileFormat {
        id: 29_650_319,
        source_type: SourceType::Wikidata,
        name: "PIM",
        extensions: &["pim"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
