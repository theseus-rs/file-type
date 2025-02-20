use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
