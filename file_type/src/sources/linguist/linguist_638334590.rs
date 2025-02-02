use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_638334590: FileFormat = FileFormat {
    id: 638_334_590,
    source_type: SourceType::Linguist,
    name: "Mustache",
    extensions: &["mustache"],
    media_types: &["text/x-smarty"],
    internal_signatures: &[],
    related_formats: &[],
};
