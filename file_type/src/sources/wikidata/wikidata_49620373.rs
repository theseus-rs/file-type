use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_49620373: FileType = FileType {
    file_format: &FileFormat {
        id: 49_620_373,
        source_type: SourceType::Wikidata,
        name: "Revit Workspace",
        extensions: &["rws"],
        media_types: &["application/octet-stream"],
        signatures: &[],
        related_formats: &[],
    },
};
