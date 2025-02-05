use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_100: FileFormat = FileFormat {
    id: 100,
    source_type: SourceType::Linguist,
    name: "Elixir",
    extensions: &["ex", "exs"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
