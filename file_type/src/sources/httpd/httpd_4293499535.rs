use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_4293499535: FileFormat = FileFormat {
    id: 4_293_499_535,
    source_type: SourceType::Httpd,
    name: "dvb service",
    extensions: &["svc"],
    media_types: &["application/vnd.dvb.service"],
    signatures: &[],
    related_formats: &[],
};
