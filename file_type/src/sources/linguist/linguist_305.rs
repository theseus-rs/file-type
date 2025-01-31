use crate::format::FileFormat;

pub(crate) const LINGUIST_305: FileFormat = FileFormat {
    id: 305,
    puid: "linguist/305",
    name: "QML",
    extensions: &["qbs", "qml"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
