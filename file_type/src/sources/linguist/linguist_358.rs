use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_358: FileFormat = FileFormat {
    id: 358,
    source_type: SourceType::Linguist,
    name: "Stata",
    extensions: &["ado", "do", "doh", "ihlp", "mata", "matah", "sthlp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
