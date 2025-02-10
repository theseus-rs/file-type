use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1022638803: FileType = FileType {
    file_format: &FileFormat {
        id: 1_022_638_803,
        source_type: SourceType::Iana,
        name: "vnd.openxmlformats-officedocument.drawingml.diagramLayout+xml",
        extensions: &[],
        media_types: &["application/vnd.openxmlformats-officedocument.drawingml.diagramLayout+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
