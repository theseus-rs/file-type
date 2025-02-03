use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_393: FileFormat = FileFormat {
    id: 393,
    source_type: SourceType::Linguist,
    name: "Wavefront Object",
    extensions: &["obj"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
