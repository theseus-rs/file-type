use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3562786460: FileFormat = FileFormat {
    id: 3_562_786_460,
    source_type: SourceType::Iana,
    name: "H224",
    extensions: &[],
    media_types: &["application/H224"],
    signatures: &[],
    related_formats: &[],
};
