use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_62414914: FileFormat = FileFormat {
    id: 62_414_914,
    source_type: SourceType::Wikidata,
    name: "Quattro Pro Spreadsheet, version 9",
    extensions: &["qpw"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
