use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_5448342: FileType = FileType {
    file_format: &FileFormat {
        id: 5_448_342,
        source_type: SourceType::Wikidata,
        name: "File change log",
        extensions: &["log"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
