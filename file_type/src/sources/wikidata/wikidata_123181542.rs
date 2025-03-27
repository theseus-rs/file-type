use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_123181542: FileType = FileType {
    file_format: &FileFormat {
        id: 123_181_542,
        source_type: SourceType::Wikidata,
        name: "RDML",
        extensions: &[],
        media_types: &["application/zip"],
        signatures: &[],
        related_formats: &[],
    },
};
