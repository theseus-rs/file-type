use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_299: FileFormat = FileFormat {
    id: 299,
    source_type: SourceType::Linguist,
    name: "Puppet",
    extensions: &["pp"],
    media_types: &["text/x-puppet"],
    internal_signatures: &[],
    related_formats: &[],
};
