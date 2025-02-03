use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_10045809873570187998: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "set payment initiation",
    extensions: &["setpay"],
    media_types: &["application/set-payment-initiation"],
    internal_signatures: &[],
    related_formats: &[],
};
