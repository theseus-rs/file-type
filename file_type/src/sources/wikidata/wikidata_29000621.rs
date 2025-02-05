use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29000621: FileFormat = FileFormat {
    id: 29_000_621,
    source_type: SourceType::Wikidata,
    name: "WinHex.pos",
    extensions: &["pos"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
