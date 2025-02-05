use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3752320099: FileFormat = FileFormat {
    id: 3_752_320_099,
    source_type: SourceType::Iana,
    name: "vnd.dvb.notif-ia-registration-request+xml",
    extensions: &[],
    media_types: &["application/vnd.dvb.notif-ia-registration-request+xml"],
    signatures: &[],
    related_formats: &[],
};
