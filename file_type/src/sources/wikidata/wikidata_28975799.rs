use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28975799: FileType = FileType {
    file_format: &FileFormat {
        id: 28_975_799,
        source_type: SourceType::Wikidata,
        name: "FACT",
        extensions: &["fact"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
