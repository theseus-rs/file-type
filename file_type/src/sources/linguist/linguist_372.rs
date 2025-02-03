use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_372: FileFormat = FileFormat {
    id: 372,
    source_type: SourceType::Linguist,
    name: "Text",
    extensions: &["fr", "nb", "ncl", "no", "txt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
