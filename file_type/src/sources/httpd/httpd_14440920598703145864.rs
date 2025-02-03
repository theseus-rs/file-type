use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_14440920598703145864: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "dvb subtitle",
    extensions: &["sub"],
    media_types: &["text/vnd.dvb.subtitle"],
    internal_signatures: &[],
    related_formats: &[],
};
