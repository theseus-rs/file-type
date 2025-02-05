use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_606708469: FileFormat = FileFormat {
    id: 606_708_469,
    source_type: SourceType::Linguist,
    name: "Tact",
    extensions: &["tact"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
