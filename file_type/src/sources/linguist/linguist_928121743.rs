use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_928121743: FileFormat = FileFormat {
    id: 928_121_743,
    source_type: SourceType::Linguist,
    name: "HolyC",
    extensions: &["hc"],
    media_types: &["text/x-csrc"],
    signatures: &[],
    related_formats: &[],
};
