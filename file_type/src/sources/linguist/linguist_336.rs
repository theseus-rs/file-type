use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_336: FileFormat = FileFormat {
    id: 336,
    source_type: SourceType::Linguist,
    name: "STON",
    extensions: &["ston"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
