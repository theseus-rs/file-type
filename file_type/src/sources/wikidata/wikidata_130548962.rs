use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_130548962: FileFormat = FileFormat {
    id: 130_548_962,
    puid: "wikidata/130548962",
    name: "QML file format",
    extensions: &["qbs", "qbs", "qml", "qml"],
    media_types: &[
        "application/x-qml",
        "application/x-qml",
        "application/x-qt.qbs+qml",
        "application/x-qt.qbs+qml",
    ],
    internal_signatures: &[],
    related_formats: &[],
};
