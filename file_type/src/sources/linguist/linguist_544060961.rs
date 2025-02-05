use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_544060961: FileFormat = FileFormat {
    id: 544_060_961,
    source_type: SourceType::Linguist,
    name: "Valve Data Format",
    extensions: &["vdf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
