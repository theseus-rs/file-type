use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2459910717: FileType = FileType {
    file_format: &FileFormat {
        id: 2_459_910_717,
        source_type: SourceType::Iana,
        name: "vnd.openxmlformats-officedocument.vmlDrawing",
        extensions: &[],
        media_types: &["application/vnd.openxmlformats-officedocument.vmlDrawing"],
        signatures: &[],
        related_formats: &[],
    },
};
