use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_400: FileFormat = FileFormat {
    id: 400,
    source_type: SourceType::Linguist,
    name: "XPages",
    extensions: &["xsp-config", "xsp.metadata"],
    media_types: &["text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
