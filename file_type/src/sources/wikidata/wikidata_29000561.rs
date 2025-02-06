use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29000561: FileFormat = FileFormat {
    id: 29_000_561,
    source_type: SourceType::Wikidata,
    name: "Kryoflux Stream",
    extensions: &["raw"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
