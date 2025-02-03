use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_62414914: FileFormat = FileFormat {
    id: 62_414_914,
    source_type: SourceType::Wikidata,
    name: "Quattro Pro Spreadsheet, version 9",
    extensions: &["qpw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
