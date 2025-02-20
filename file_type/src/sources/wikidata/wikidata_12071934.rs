use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_12071934: FileType = FileType {
    file_format: &FileFormat {
        id: 12_071_934,
        source_type: SourceType::Wikidata,
        name: "nl",
        extensions: &["nl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
