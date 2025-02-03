use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2186089726: FileFormat = FileFormat {
    id: 2_186_089_726,
    source_type: SourceType::Iana,
    name: "vnd.cyan.dean.root+xml",
    extensions: &[],
    media_types: &["application/vnd.cyan.dean.root+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
