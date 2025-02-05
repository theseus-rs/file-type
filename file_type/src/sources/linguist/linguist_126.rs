use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_126: FileFormat = FileFormat {
    id: 126,
    source_type: SourceType::Linguist,
    name: "Genshi",
    extensions: &["kid"],
    media_types: &["text/xml"],
    signatures: &[],
    related_formats: &[],
};
