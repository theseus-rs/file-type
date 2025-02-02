use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_440182480: FileFormat = FileFormat {
    id: 440_182_480,
    source_type: SourceType::Linguist,
    name: "Roc",
    extensions: &["roc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
