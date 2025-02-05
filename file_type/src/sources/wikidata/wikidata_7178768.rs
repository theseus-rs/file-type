use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_7178768: FileFormat = FileFormat {
    id: 7_178_768,
    source_type: SourceType::Wikidata,
    name: "Petri Net Markup Language",
    extensions: &["pnml"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
