use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29000745: FileFormat = FileFormat {
    id: 29_000_745,
    source_type: SourceType::Wikidata,
    name: "MultiGen Flight",
    extensions: &["flt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
