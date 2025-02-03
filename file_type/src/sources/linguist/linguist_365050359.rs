use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_365050359: FileFormat = FileFormat {
    id: 365_050_359,
    source_type: SourceType::Linguist,
    name: "Luau",
    extensions: &["luau"],
    media_types: &["text/x-lua"],
    internal_signatures: &[],
    related_formats: &[],
};
