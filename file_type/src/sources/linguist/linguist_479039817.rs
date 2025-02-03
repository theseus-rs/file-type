use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_479039817: FileFormat = FileFormat {
    id: 479_039_817,
    source_type: SourceType::Linguist,
    name: "HTML+Razor",
    extensions: &["cshtml", "razor"],
    media_types: &["text/html"],
    internal_signatures: &[],
    related_formats: &[],
};
