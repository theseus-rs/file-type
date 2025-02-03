use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1970025208: FileFormat = FileFormat {
    id: 1_970_025_208,
    source_type: SourceType::Iana,
    name: "vnd.s3sms",
    extensions: &[],
    media_types: &["application/vnd.s3sms"],
    internal_signatures: &[],
    related_formats: &[],
};
