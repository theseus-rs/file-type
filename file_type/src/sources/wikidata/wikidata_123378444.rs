use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_123378444: FileFormat = FileFormat {
    id: 123_378_444,
    source_type: SourceType::Wikidata,
    name: "Caligari Amiga file",
    extensions: &["sob"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
