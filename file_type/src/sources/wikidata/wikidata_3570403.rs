use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_3570403: FileFormat = FileFormat {
    id: 3_570_403,
    source_type: SourceType::Wikidata,
    name: "Office Open XML Spreadsheet Document, ECMA-376 1st Edition",
    extensions: &["xlsx"],
    media_types: &["application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"],
    internal_signatures: &[],
    related_formats: &[],
};
