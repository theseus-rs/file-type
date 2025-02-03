use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_113481871: FileFormat = FileFormat {
    id: 113_481_871,
    source_type: SourceType::Wikidata,
    name: "Calc602 Spreadsheet file or backup file",
    extensions: &["bak", "tc6"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
