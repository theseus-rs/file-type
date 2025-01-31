use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_112596194: FileFormat = FileFormat {
    id: 112_596_194,
    puid: "wikidata/112596194",
    name: "Office Open XML Spreadsheet Document, Strict, ISO/IEC 29500:2016",
    extensions: &["xlsm", "xlsx", "xltm", "xltx"],
    media_types: &[
        "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
        "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
        "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
        "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
    ],
    internal_signatures: &[],
    related_formats: &[],
};
