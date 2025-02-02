use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_130548962: FileFormat = FileFormat {
    id: 130_548_962,
    source_type: SourceType::Wikidata,
    name: "QML file format",
    extensions: &["qbs", "qml"],
    media_types: &["application/x-qml", "application/x-qt.qbs+qml"],
    internal_signatures: &[],
    related_formats: &[],
};
