use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_646424281: FileFormat = FileFormat {
    id: 646_424_281,
    source_type: SourceType::Linguist,
    name: "Zig",
    extensions: &["zig", "zig.zon"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
