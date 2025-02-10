use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_112652264: FileType = FileType {
    file_format: &FileFormat {
        id: 112_652_264,
        source_type: SourceType::Wikidata,
        name: "Autodesk SketchBook Animation File",
        extensions: &["skba"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
