use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_838252715: FileFormat = FileFormat {
    id: 838_252_715,
    source_type: SourceType::Linguist,
    name: "Ink",
    extensions: &["ink"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
