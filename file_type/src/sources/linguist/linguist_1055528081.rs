use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_1055528081: FileFormat = FileFormat {
    id: 1_055_528_081,
    source_type: SourceType::Linguist,
    name: "Bikeshed",
    extensions: &["bs"],
    media_types: &["text/html"],
    signatures: &[],
    related_formats: &[],
};
