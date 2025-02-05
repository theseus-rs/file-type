use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_719519: FileFormat = FileFormat {
    id: 719_519,
    source_type: SourceType::Wikidata,
    name: "Forsyth–Edwards Notation",
    extensions: &["fen"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
