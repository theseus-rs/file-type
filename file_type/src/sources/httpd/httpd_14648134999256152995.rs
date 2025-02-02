use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_14648134999256152995: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "dvb service",
    extensions: &["svc"],
    media_types: &["application/vnd.dvb.service"],
    internal_signatures: &[],
    related_formats: &[],
};
