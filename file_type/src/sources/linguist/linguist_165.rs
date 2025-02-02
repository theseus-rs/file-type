use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_165: FileFormat = FileFormat {
    id: 165,
    source_type: SourceType::Linguist,
    name: "Idris",
    extensions: &["idr", "lidr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
