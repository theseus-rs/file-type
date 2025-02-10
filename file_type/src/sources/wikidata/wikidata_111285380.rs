use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111285380: FileType = FileType {
    file_format: &FileFormat {
        id: 111_285_380,
        source_type: SourceType::Wikidata,
        name: "Ensoniq EPS family disk image",
        extensions: &["gkh"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
