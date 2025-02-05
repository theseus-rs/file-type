use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_292377326: FileFormat = FileFormat {
    id: 292_377_326,
    source_type: SourceType::Linguist,
    name: "Velocity Template Language",
    extensions: &["vtl"],
    media_types: &["text/velocity"],
    signatures: &[],
    related_formats: &[],
};
