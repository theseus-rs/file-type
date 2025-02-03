use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_501733551: FileFormat = FileFormat {
    id: 501_733_551,
    source_type: SourceType::Iana,
    name: "vnd.svd",
    extensions: &[],
    media_types: &["application/vnd.svd"],
    internal_signatures: &[],
    related_formats: &[],
};
