use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_112596194: FileFormat = FileFormat {
    id: 112_596_194,
    source_type: SourceType::Wikidata,
    name: "Office Open XML Spreadsheet Document, Strict, ISO/IEC 29500:2016",
    extensions: &["xlsm", "xlsx", "xltm", "xltx"],
    media_types: &["application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"],
    internal_signatures: &[],
    related_formats: &[],
};
