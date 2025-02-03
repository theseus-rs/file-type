use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_156: FileFormat = FileFormat {
    id: 156,
    source_type: SourceType::Linguist,
    name: "Harbour",
    extensions: &["hb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
