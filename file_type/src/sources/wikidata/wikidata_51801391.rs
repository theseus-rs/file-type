use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_51801391: FileType = FileType {
    file_format: &FileFormat {
        id: 51_801_391,
        source_type: SourceType::Wikidata,
        name: "AutoCAD Xref Log",
        extensions: &["xlg"],
        media_types: &["application/octet-stream"],
        signatures: &[],
        related_formats: &[],
    },
};
