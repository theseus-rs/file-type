use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3473425801: FileType = FileType {
    file_format: &FileFormat {
        id: 3_473_425_801,
        source_type: SourceType::Iana,
        name: "vnd.clonk.c4group",
        extensions: &[],
        media_types: &["application/vnd.clonk.c4group"],
        signatures: &[],
        related_formats: &[],
    },
};
