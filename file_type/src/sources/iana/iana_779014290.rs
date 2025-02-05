use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_779014290: FileFormat = FileFormat {
    id: 779_014_290,
    source_type: SourceType::Iana,
    name: "vnd.japannet-registration-wakeup",
    extensions: &[],
    media_types: &["application/vnd.japannet-registration-wakeup"],
    signatures: &[],
    related_formats: &[],
};
