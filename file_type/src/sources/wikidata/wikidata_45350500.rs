use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_45350500: FileFormat = FileFormat {
    id: 45_350_500,
    source_type: SourceType::Wikidata,
    name: "Lotus 1-2-3 Spreadsheet Formatting File, version 3",
    extensions: &["fm3"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
