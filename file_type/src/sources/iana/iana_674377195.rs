use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_674377195: FileFormat = FileFormat {
    id: 674_377_195,
    source_type: SourceType::Iana,
    name: "vnd.3gpp2.sms",
    extensions: &[],
    media_types: &["application/vnd.3gpp2.sms"],
    internal_signatures: &[],
    related_formats: &[],
};
