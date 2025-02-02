use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_261: FileFormat = FileFormat {
    id: 261,
    source_type: SourceType::Linguist,
    name: "Opa",
    extensions: &["opa"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
