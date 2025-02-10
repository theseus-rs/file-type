use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1747625988: FileType = FileType {
    file_format: &FileFormat {
        id: 1_747_625_988,
        source_type: SourceType::Iana,
        name: "set-payment-initiation",
        extensions: &[],
        media_types: &["application/set-payment-initiation"],
        signatures: &[],
        related_formats: &[],
    },
};
