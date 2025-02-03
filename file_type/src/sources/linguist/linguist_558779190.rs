use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_558779190: FileFormat = FileFormat {
    id: 558_779_190,
    source_type: SourceType::Linguist,
    name: "Sweave",
    extensions: &["rnw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
