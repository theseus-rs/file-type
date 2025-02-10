use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_113532977: FileType = FileType {
    file_format: &FileFormat {
        id: 113_532_977,
        source_type: SourceType::Wikidata,
        name: "Wordcraft Chapter File",
        extensions: &["001"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
