use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_29000593: FileType = FileType {
    file_format: &FileFormat {
        id: 29_000_593,
        source_type: SourceType::Wikidata,
        name: "HFS Plus Journal",
        extensions: &["journal"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
