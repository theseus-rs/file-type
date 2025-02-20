use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_51954279: FileType = FileType {
    file_format: &FileFormat {
        id: 51_954_279,
        source_type: SourceType::Wikidata,
        name: "Autodesk Animator CEL File Format",
        extensions: &["cel"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
