use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_130357389: FileFormat = FileFormat {
    id: 130_357_389,
    source_type: SourceType::Wikidata,
    name: "MOOCode file format",
    extensions: &["moo"],
    media_types: &["text/x-moocode"],
    signatures: &[],
    related_formats: &[],
};
