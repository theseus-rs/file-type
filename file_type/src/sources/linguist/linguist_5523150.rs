use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_5523150: FileFormat = FileFormat {
    id: 5_523_150,
    source_type: SourceType::Linguist,
    name: "Glimmer JS",
    extensions: &["gjs"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
