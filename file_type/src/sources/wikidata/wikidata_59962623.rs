use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_59962623: FileType = FileType {
    file_format: &FileFormat {
        id: 59_962_623,
        source_type: SourceType::Wikidata,
        name: "Autodesk Animator (FlicLib)",
        extensions: &["fli"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
