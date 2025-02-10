use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_7886963: FileType = FileType {
    file_format: &FileFormat {
        id: 7_886_963,
        source_type: SourceType::Wikidata,
        name: "Uniqueness Database File",
        extensions: &["udf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
