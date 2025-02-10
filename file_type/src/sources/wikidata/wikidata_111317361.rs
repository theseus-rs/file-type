use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111317361: FileType = FileType {
    file_format: &FileFormat {
        id: 111_317_361,
        source_type: SourceType::Wikidata,
        name: "MAUD sample format",
        extensions: &["maud"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
