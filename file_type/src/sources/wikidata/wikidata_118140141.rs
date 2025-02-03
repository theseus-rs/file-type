use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_118140141: FileFormat = FileFormat {
    id: 118_140_141,
    source_type: SourceType::Wikidata,
    name: "Serenade Schematic File",
    extensions: &["sch"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
