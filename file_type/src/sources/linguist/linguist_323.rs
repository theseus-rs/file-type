use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_323: FileFormat = FileFormat {
    id: 323,
    source_type: SourceType::Linguist,
    name: "RenderScript",
    extensions: &["rs", "rsh"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
