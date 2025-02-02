use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_603371597: FileFormat = FileFormat {
    id: 603_371_597,
    source_type: SourceType::Linguist,
    name: "V",
    extensions: &["v"],
    media_types: &["text/x-go"],
    internal_signatures: &[],
    related_formats: &[],
};
