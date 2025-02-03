use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_258: FileFormat = FileFormat {
    id: 258,
    source_type: SourceType::Linguist,
    name: "Objective-C++",
    extensions: &["mm"],
    media_types: &["text/x-objectivec"],
    internal_signatures: &[],
    related_formats: &[],
};
