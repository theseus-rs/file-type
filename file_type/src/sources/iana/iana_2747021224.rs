use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2747021224: FileType = FileType {
    file_format: &FileFormat {
        id: 2_747_021_224,
        source_type: SourceType::Iana,
        name: "jwt",
        extensions: &[],
        media_types: &["application/jwt"],
        signatures: &[],
        related_formats: &[],
    },
};
