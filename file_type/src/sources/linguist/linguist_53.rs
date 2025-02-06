use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_53: FileFormat = FileFormat {
    id: 53,
    source_type: SourceType::Linguist,
    name: "CartoCSS",
    extensions: &["mss"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
