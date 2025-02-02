use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_47202526: FileFormat = FileFormat {
    id: 47_202_526,
    source_type: SourceType::Wikidata,
    name: "AppleWorks Spreadsheet file format version 6",
    extensions: &["cwk"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
