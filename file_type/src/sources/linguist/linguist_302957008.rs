use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_302957008: FileFormat = FileFormat {
    id: 302_957_008,
    source_type: SourceType::Linguist,
    name: "GN",
    extensions: &["gn", "gni"],
    media_types: &["text/x-python"],
    internal_signatures: &[],
    related_formats: &[],
};
