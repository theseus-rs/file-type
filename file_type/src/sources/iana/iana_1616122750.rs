use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1616122750: FileType = FileType {
    file_format: &FileFormat {
        id: 1_616_122_750,
        source_type: SourceType::Iana,
        name: "pkcs10",
        extensions: &[],
        media_types: &["application/pkcs10"],
        signatures: &[],
        related_formats: &[],
    },
};
