use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27967445: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_445,
        source_type: SourceType::Wikidata,
        name: "Autodesk Animator Pro FLIC",
        extensions: &["flc"],
        media_types: &["video/flc"],
        signatures: &[],
        related_formats: &[],
    },
};
