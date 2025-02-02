use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_123378444: FileFormat = FileFormat {
    id: 123_378_444,
    source_type: SourceType::Wikidata,
    name: "Caligari Amiga file",
    extensions: &["sob"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
