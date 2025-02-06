use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3693945932: FileFormat = FileFormat {
    id: 3_693_945_932,
    source_type: SourceType::Iana,
    name: "vnd.ipld.dag-json",
    extensions: &[],
    media_types: &["application/vnd.ipld.dag-json"],
    signatures: &[],
    related_formats: &[],
};
