use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_152: FileFormat = FileFormat {
    id: 152,
    source_type: SourceType::Linguist,
    name: "HTTP",
    extensions: &["http"],
    media_types: &["message/http"],
    internal_signatures: &[],
    related_formats: &[],
};
