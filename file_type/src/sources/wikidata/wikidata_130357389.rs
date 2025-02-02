use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_130357389: FileFormat = FileFormat {
    id: 130_357_389,
    source_type: SourceType::Wikidata,
    name: "MOOCode file format",
    extensions: &["moo"],
    media_types: &["text/x-moocode"],
    internal_signatures: &[],
    related_formats: &[],
};
