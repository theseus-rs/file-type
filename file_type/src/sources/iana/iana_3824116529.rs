use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3824116529: FileType = FileType {
    file_format: &FileFormat {
        id: 3_824_116_529,
        source_type: SourceType::Iana,
        name: "vnd.proteus.magazine",
        extensions: &[],
        media_types: &["application/vnd.proteus.magazine"],
        signatures: &[],
        related_formats: &[],
    },
};
