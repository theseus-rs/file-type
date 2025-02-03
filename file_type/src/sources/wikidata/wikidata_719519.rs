use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_719519: FileFormat = FileFormat {
    id: 719_519,
    source_type: SourceType::Wikidata,
    name: "Forsyth–Edwards Notation",
    extensions: &["fen"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
