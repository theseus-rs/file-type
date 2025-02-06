use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_113481871: FileFormat = FileFormat {
    id: 113_481_871,
    source_type: SourceType::Wikidata,
    name: "Calc602 Spreadsheet file or backup file",
    extensions: &["bak", "tc6"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
