use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_598917541: FileFormat = FileFormat {
    id: 598_917_541,
    source_type: SourceType::Linguist,
    name: "OpenStep Property List",
    extensions: &["glyphs", "plist"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
