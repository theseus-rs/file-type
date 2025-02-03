use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2327811077702530899: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "dvb subtitle",
    extensions: &["sub"],
    media_types: &["image/vnd.dvb.subtitle"],
    internal_signatures: &[],
    related_formats: &[],
};
