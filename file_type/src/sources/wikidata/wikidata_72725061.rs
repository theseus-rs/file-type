use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_72725061: FileType = FileType {
    file_format: &FileFormat {
        id: 72_725_061,
        source_type: SourceType::Wikidata,
        name: "NATO Secondary Imagery Format",
        extensions: &["nsf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
