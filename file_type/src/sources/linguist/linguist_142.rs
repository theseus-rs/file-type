use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_142: FileFormat = FileFormat {
    id: 142,
    source_type: SourceType::Linguist,
    name: "Groovy",
    extensions: &["groovy", "grt", "gtpl", "gvy"],
    media_types: &["text/x-groovy"],
    internal_signatures: &[],
    related_formats: &[],
};
