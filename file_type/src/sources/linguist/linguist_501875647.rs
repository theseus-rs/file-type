use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_501875647: FileFormat = FileFormat {
    id: 501_875_647,
    source_type: SourceType::Linguist,
    name: "ReScript",
    extensions: &["res"],
    media_types: &["text/x-rustsrc"],
    internal_signatures: &[],
    related_formats: &[],
};
