use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_123299060: FileType = FileType {
    file_format: &FileFormat {
        id: 123_299_060,
        source_type: SourceType::Wikidata,
        name: "Ancestry.com Family Tree Database",
        extensions: &["aft"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
