use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_51239111: FileFormat = FileFormat {
    id: 51_239_111,
    source_type: SourceType::Linguist,
    name: "OASv3-yaml",
    extensions: &["yaml", "yml"],
    media_types: &["text/x-yaml"],
    internal_signatures: &[],
    related_formats: &[],
};
