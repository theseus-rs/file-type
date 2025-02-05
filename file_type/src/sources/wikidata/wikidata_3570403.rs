use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_3570403: FileFormat = FileFormat {
    id: 3_570_403,
    source_type: SourceType::Wikidata,
    name: "Office Open XML Spreadsheet Document, ECMA-376 1st Edition",
    extensions: &["xlsx"],
    media_types: &["application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"],
    signatures: &[],
    related_formats: &[],
};
