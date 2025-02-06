use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_305: FileFormat = FileFormat {
    id: 305,
    source_type: SourceType::Linguist,
    name: "QML",
    extensions: &["qbs", "qml"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
