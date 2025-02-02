use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_592853203: FileFormat = FileFormat {
    id: 592_853_203,
    source_type: SourceType::Linguist,
    name: "RouterOS Script",
    extensions: &["rsc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
