use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_83159841: FileType = FileType {
    file_format: &FileFormat {
        id: 83_159_841,
        source_type: SourceType::Wikidata,
        name: "CRN",
        extensions: &["crn"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
