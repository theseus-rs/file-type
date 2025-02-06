use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_34: FileFormat = FileFormat {
    id: 34,
    source_type: SourceType::Linguist,
    name: "BlitzBasic",
    extensions: &["bb", "decls"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
