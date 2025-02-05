use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_58526909: FileFormat = FileFormat {
    id: 58_526_909,
    source_type: SourceType::Wikidata,
    name: "SuperCalc Spreadsheet, version 1",
    extensions: &["cal"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
