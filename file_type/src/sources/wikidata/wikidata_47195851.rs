use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_47195851: FileFormat = FileFormat {
    id: 47_195_851,
    source_type: SourceType::Wikidata,
    name: "AppleWorks Spreadsheet file format, version 5",
    extensions: &["cwk"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
