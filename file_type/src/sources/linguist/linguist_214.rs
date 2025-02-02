use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_214: FileFormat = FileFormat {
    id: 214,
    source_type: SourceType::Linguist,
    name: "M",
    extensions: &["m", "mumps"],
    media_types: &["text/x-mumps"],
    internal_signatures: &[],
    related_formats: &[],
};
