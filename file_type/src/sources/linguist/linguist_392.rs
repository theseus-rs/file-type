use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_392: FileFormat = FileFormat {
    id: 392,
    source_type: SourceType::Linguist,
    name: "Wavefront Material",
    extensions: &["mtl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
