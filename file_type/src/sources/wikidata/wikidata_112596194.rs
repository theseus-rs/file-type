use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_112596194: FileFormat = FileFormat {
    id: 112_596_194,
    source_type: SourceType::Wikidata,
    name: "Office Open XML Spreadsheet Document, Strict, ISO/IEC 29500:2016",
    extensions: &["xlsm", "xlsx", "xltm", "xltx"],
    media_types: &["application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"],
    signatures: &[],
    related_formats: &[],
};
