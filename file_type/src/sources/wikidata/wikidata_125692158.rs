use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_125692158: FileFormat = FileFormat {
    id: 125_692_158,
    source_type: SourceType::Wikidata,
    name: "OpenDocument Spreadsheet Template",
    extensions: &["ots"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
