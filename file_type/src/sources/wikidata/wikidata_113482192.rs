use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_113482192: FileFormat = FileFormat {
    id: 113_482_192,
    source_type: SourceType::Wikidata,
    name: "Calc602 Spreadsheet file or backup file v.1.00",
    extensions: &["bak", "tc6"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
