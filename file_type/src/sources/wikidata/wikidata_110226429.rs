use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_110226429: FileFormat = FileFormat {
    id: 110_226_429,
    source_type: SourceType::Wikidata,
    name: "ELAN Preference File",
    extensions: &["pfsx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
