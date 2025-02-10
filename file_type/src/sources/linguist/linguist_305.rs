use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_305: FileType = FileType {
    file_format: &FileFormat {
        id: 305,
        source_type: SourceType::Linguist,
        name: "QML",
        extensions: &["qbs", "qml"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
