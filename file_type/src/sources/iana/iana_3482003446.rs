use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3482003446: FileType = FileType {
    file_format: &FileFormat {
        id: 3_482_003_446,
        source_type: SourceType::Iana,
        name: "atomicmail",
        extensions: &[],
        media_types: &["application/atomicmail"],
        signatures: &[],
        related_formats: &[],
    },
};
