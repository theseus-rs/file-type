use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1747625988: FileFormat = FileFormat {
    id: 1_747_625_988,
    source_type: SourceType::Httpd,
    name: "set payment initiation",
    extensions: &["setpay"],
    media_types: &["application/set-payment-initiation"],
    internal_signatures: &[],
    related_formats: &[],
};
