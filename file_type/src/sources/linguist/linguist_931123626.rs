use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_931123626: FileFormat = FileFormat {
    id: 931_123_626,
    source_type: SourceType::Linguist,
    name: "KDL",
    extensions: &["kdl"],
    media_types: &["text/x-yacas"],
    internal_signatures: &[],
    related_formats: &[],
};
