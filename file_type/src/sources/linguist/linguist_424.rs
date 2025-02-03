use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_424: FileFormat = FileFormat {
    id: 424,
    source_type: SourceType::Linguist,
    name: "CSON",
    extensions: &["cson"],
    media_types: &["text/x-coffeescript"],
    internal_signatures: &[],
    related_formats: &[],
};
