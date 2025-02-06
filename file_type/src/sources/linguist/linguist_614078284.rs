use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_614078284: FileFormat = FileFormat {
    id: 614_078_284,
    source_type: SourceType::Linguist,
    name: "Xonsh",
    extensions: &["xsh"],
    media_types: &["text/x-python"],
    signatures: &[],
    related_formats: &[],
};
