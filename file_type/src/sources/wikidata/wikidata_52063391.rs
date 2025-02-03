use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_52063391: FileFormat = FileFormat {
    id: 52_063_391,
    source_type: SourceType::Wikidata,
    name: "SuperCalc Spreadsheet, version 5",
    extensions: &["cal"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
