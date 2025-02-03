use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_384: FileFormat = FileFormat {
    id: 384,
    source_type: SourceType::Linguist,
    name: "VCL",
    extensions: &["vcl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
