use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_791486801: FileType = FileType {
    file_format: &FileFormat {
        id: 791_486_801,
        source_type: SourceType::Iana,
        name: "exi",
        extensions: &[],
        media_types: &["application/exi"],
        signatures: &[],
        related_formats: &[],
    },
};
