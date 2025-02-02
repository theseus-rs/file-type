use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_339: FileFormat = FileFormat {
    id: 339,
    source_type: SourceType::Linguist,
    name: "SaltStack",
    extensions: &["sls"],
    media_types: &["text/x-yaml"],
    internal_signatures: &[],
    related_formats: &[],
};
