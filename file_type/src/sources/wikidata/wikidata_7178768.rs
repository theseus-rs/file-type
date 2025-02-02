use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_7178768: FileFormat = FileFormat {
    id: 7_178_768,
    source_type: SourceType::Wikidata,
    name: "Petri Net Markup Language",
    extensions: &["pnml"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
