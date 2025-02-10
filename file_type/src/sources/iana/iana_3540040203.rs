use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3540040203: FileType = FileType {
    file_format: &FileFormat {
        id: 3_540_040_203,
        source_type: SourceType::Iana,
        name: "vnd.xmpie.cpkg",
        extensions: &[],
        media_types: &["application/vnd.xmpie.cpkg"],
        signatures: &[],
        related_formats: &[],
    },
};
