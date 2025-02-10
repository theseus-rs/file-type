use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_131418899: FileType = FileType {
    file_format: &FileFormat {
        id: 131_418_899,
        source_type: SourceType::Wikidata,
        name: "Web IDL file format",
        extensions: &["webidl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
