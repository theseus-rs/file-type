use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_125692158: FileFormat = FileFormat {
    id: 125_692_158,
    source_type: SourceType::Wikidata,
    name: "OpenDocument Spreadsheet Template",
    extensions: &["ots"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
