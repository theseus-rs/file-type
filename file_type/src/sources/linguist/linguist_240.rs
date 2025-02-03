use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_240: FileFormat = FileFormat {
    id: 240,
    source_type: SourceType::Linguist,
    name: "NCL",
    extensions: &["ncl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
