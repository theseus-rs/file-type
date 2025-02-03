use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_29167429: FileFormat = FileFormat {
    id: 29_167_429,
    source_type: SourceType::Wikidata,
    name: "NovaMind",
    extensions: &["nmind"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
