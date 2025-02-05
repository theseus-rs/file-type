use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_632745969: FileFormat = FileFormat {
    id: 632_745_969,
    source_type: SourceType::Linguist,
    name: "Wollok",
    extensions: &["wlk"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
