use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3450423353: FileFormat = FileFormat {
    id: 3_450_423_353,
    source_type: SourceType::Httpd,
    name: "cluetrust cartomobile config pkg",
    extensions: &["c11amz"],
    media_types: &["application/vnd.cluetrust.cartomobile-config-pkg"],
    internal_signatures: &[],
    related_formats: &[],
};
