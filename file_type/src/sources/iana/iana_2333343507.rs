use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2333343507: FileFormat = FileFormat {
    id: 2_333_343_507,
    source_type: SourceType::Iana,
    name: "samlassertion+xml",
    extensions: &[],
    media_types: &["application/samlassertion+xml"],
    signatures: &[],
    related_formats: &[],
};
