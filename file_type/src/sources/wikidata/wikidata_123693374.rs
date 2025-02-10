use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_123693374: FileType = FileType {
    file_format: &FileFormat {
        id: 123_693_374,
        source_type: SourceType::Wikidata,
        name: "Pascal unit file",
        extensions: &["pas"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
