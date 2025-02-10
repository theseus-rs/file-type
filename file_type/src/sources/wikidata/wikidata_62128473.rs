use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_62128473: FileType = FileType {
    file_format: &FileFormat {
        id: 62_128_473,
        source_type: SourceType::Wikidata,
        name: "CSV Schema",
        extensions: &["csvs"],
        media_types: &["text/csv-schema"],
        signatures: &[],
        related_formats: &[],
    },
};
