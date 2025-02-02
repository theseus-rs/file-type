use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_31: FileFormat = FileFormat {
    id: 31,
    source_type: SourceType::Linguist,
    name: "Bison",
    extensions: &["bison"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
