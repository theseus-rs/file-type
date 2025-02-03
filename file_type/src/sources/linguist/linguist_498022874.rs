use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_498022874: FileFormat = FileFormat {
    id: 498_022_874,
    source_type: SourceType::Linguist,
    name: "Rez",
    extensions: &["r"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
