use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2429679128: FileFormat = FileFormat {
    id: 2_429_679_128,
    source_type: SourceType::Iana,
    name: "vnd.mcd",
    extensions: &[],
    media_types: &["application/vnd.mcd"],
    internal_signatures: &[],
    related_formats: &[],
};
