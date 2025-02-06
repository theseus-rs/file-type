use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_689870117: FileFormat = FileFormat {
    id: 689_870_117,
    source_type: SourceType::Iana,
    name: "cea-2018+xml",
    extensions: &[],
    media_types: &["application/cea-2018+xml"],
    signatures: &[],
    related_formats: &[],
};
