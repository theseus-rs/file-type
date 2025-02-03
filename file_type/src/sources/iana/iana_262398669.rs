use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_262398669: FileFormat = FileFormat {
    id: 262_398_669,
    source_type: SourceType::Iana,
    name: "vnd.dvb.dvbisl+xml",
    extensions: &[],
    media_types: &["application/vnd.dvb.dvbisl+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
