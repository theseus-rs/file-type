use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_288: FileFormat = FileFormat {
    id: 288,
    source_type: SourceType::Linguist,
    name: "Pod",
    extensions: &["pod"],
    media_types: &["text/x-perl"],
    internal_signatures: &[],
    related_formats: &[],
};
