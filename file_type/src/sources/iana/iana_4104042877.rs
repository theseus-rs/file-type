use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4104042877: FileFormat = FileFormat {
    id: 4_104_042_877,
    source_type: SourceType::Iana,
    name: "davmount+xml",
    extensions: &[],
    media_types: &["application/davmount+xml"],
    signatures: &[],
    related_formats: &[],
};
