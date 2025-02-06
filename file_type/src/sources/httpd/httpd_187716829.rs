use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_187716829: FileFormat = FileFormat {
    id: 187_716_829,
    source_type: SourceType::Httpd,
    name: "cluetrust cartomobile config",
    extensions: &["c11amc"],
    media_types: &["application/vnd.cluetrust.cartomobile-config"],
    signatures: &[],
    related_formats: &[],
};
