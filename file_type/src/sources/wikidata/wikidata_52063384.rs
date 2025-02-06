use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_52063384: FileFormat = FileFormat {
    id: 52_063_384,
    source_type: SourceType::Wikidata,
    name: "SuperCalc Spreadsheet, version 4",
    extensions: &["cal"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
