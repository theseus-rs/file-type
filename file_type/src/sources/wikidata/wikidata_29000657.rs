use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29000657: FileFormat = FileFormat {
    id: 29_000_657,
    source_type: SourceType::Wikidata,
    name: "Polygon data file",
    extensions: &["poly"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
