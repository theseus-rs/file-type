use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_1747625988: FileType = FileType {
    file_format: &FileFormat {
        id: 1_747_625_988,
        source_type: SourceType::Httpd,
        name: "set payment initiation",
        extensions: &["setpay"],
        media_types: &["application/set-payment-initiation"],
        signatures: &[],
        related_formats: &[],
    },
};
