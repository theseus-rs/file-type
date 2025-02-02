use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_332: FileFormat = FileFormat {
    id: 332,
    source_type: SourceType::Linguist,
    name: "SQF",
    extensions: &["hqf", "sqf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
