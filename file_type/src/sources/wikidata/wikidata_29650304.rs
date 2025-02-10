use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_29650304: FileType = FileType {
    file_format: &FileFormat {
        id: 29_650_304,
        source_type: SourceType::Wikidata,
        name: "PRT scene description",
        extensions: &["dat"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
