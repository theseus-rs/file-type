use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3300626483: FileFormat = FileFormat {
    id: 3_300_626_483,
    source_type: SourceType::Iana,
    name: "vnd.iso11783-10+zip",
    extensions: &[],
    media_types: &["application/vnd.iso11783-10+zip"],
    signatures: &[],
    related_formats: &[],
};
