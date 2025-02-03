use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_993720141: FileFormat = FileFormat {
    id: 993_720_141,
    source_type: SourceType::Httpd,
    name: "dvb subtitle",
    extensions: &["sub"],
    media_types: &["text/vnd.dvb.subtitle"],
    internal_signatures: &[],
    related_formats: &[],
};
