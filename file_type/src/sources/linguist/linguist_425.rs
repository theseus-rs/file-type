use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_425: FileFormat = FileFormat {
    id: 425,
    source_type: SourceType::Linguist,
    name: "Pic",
    extensions: &["chem", "pic"],
    media_types: &["text/troff"],
    internal_signatures: &[],
    related_formats: &[],
};
