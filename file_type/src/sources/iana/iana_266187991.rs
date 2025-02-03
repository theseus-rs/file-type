use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_266187991: FileFormat = FileFormat {
    id: 266_187_991,
    source_type: SourceType::Iana,
    name: "vnd.dvb.notif-ia-registration-response+xml",
    extensions: &[],
    media_types: &["application/vnd.dvb.notif-ia-registration-response+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
