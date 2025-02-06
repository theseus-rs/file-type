use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_47195851: FileFormat = FileFormat {
    id: 47_195_851,
    source_type: SourceType::Wikidata,
    name: "AppleWorks Spreadsheet file format, version 5",
    extensions: &["cwk"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
