use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111496391: FileType = FileType {
    file_format: &FileFormat {
        id: 111_496_391,
        source_type: SourceType::Wikidata,
        name: "Visual Basic Project Workspace File",
        extensions: &["vbw"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
