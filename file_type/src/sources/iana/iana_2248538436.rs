use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2248538436: FileType = FileType {
    file_format: &FileFormat {
        id: 2_248_538_436,
        source_type: SourceType::Iana,
        name: "flexfec",
        extensions: &[],
        media_types: &["video/flexfec"],
        signatures: &[],
        related_formats: &[],
    },
};
