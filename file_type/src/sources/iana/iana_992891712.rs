use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_992891712: FileFormat = FileFormat {
    id: 992_891_712,
    source_type: SourceType::Iana,
    name: "vnd.3gpp.sms",
    extensions: &[],
    media_types: &["application/vnd.3gpp.sms"],
    signatures: &[],
    related_formats: &[],
};
