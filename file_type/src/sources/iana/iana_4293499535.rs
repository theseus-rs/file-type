use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4293499535: FileFormat = FileFormat {
    id: 4_293_499_535,
    source_type: SourceType::Iana,
    name: "vnd.dvb.service",
    extensions: &[],
    media_types: &["application/vnd.dvb.service"],
    signatures: &[],
    related_formats: &[],
};
