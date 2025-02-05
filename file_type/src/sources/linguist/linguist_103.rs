use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_103: FileFormat = FileFormat {
    id: 103,
    source_type: SourceType::Linguist,
    name: "EmberScript",
    extensions: &["em", "emberscript"],
    media_types: &["text/x-coffeescript"],
    signatures: &[],
    related_formats: &[],
};
