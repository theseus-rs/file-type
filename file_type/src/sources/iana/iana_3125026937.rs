use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3125026937: FileFormat = FileFormat {
    id: 3_125_026_937,
    source_type: SourceType::Iana,
    name: "vnd.dvb.notif-init+xml",
    extensions: &[],
    media_types: &["application/vnd.dvb.notif-init+xml"],
    signatures: &[],
    related_formats: &[],
};
