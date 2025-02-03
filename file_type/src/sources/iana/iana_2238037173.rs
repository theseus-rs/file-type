use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2238037173: FileFormat = FileFormat {
    id: 2_238_037_173,
    source_type: SourceType::Iana,
    name: "wsdl+xml",
    extensions: &[],
    media_types: &["application/wsdl+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
