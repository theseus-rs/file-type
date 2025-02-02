use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_29000621: FileFormat = FileFormat {
    id: 29_000_621,
    source_type: SourceType::Wikidata,
    name: "WinHex.pos",
    extensions: &["pos"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
