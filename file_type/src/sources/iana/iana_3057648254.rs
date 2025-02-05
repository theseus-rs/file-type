use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3057648254: FileFormat = FileFormat {
    id: 3_057_648_254,
    source_type: SourceType::Iana,
    name: "vnd.contact.cmsg",
    extensions: &[],
    media_types: &["application/vnd.contact.cmsg"],
    signatures: &[],
    related_formats: &[],
};
