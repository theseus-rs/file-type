use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_130548962: FileType = FileType {
    file_format: &FileFormat {
        id: 130_548_962,
        source_type: SourceType::Wikidata,
        name: "QML file format",
        extensions: &["qbs", "qml"],
        media_types: &["application/x-qml", "application/x-qt.qbs+qml"],
        signatures: &[],
        related_formats: &[],
    },
};
