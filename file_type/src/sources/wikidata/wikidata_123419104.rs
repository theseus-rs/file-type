use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_123419104: FileType = FileType {
    file_format: &FileFormat {
        id: 123_419_104,
        source_type: SourceType::Wikidata,
        name: "Real-time PCR Data Essential Spreadsheet Format",
        extensions: &["tsv"],
        media_types: &["text/tab-separated-values"],
        signatures: &[],
        related_formats: &[],
    },
};
