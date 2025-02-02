use crate::format::{FileFormat, SourceType};

pub(crate) const DEFAULT_1: FileFormat = FileFormat {
    id: 1,
    source_type: SourceType::Default,
    name: "Binary",
    extensions: &[],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
