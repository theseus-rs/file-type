use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_9: FileFormat = FileFormat {
    id: 9,
    source_type: SourceType::Linguist,
    name: "ATS",
    extensions: &["dats", "hats", "sats"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
