use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3391896951: FileType = FileType {
    file_format: &FileFormat {
        id: 3_391_896_951,
        source_type: SourceType::Iana,
        name: "CPIM",
        extensions: &[],
        media_types: &["message/CPIM"],
        signatures: &[],
        related_formats: &[],
    },
};
