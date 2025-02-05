use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_110226429: FileFormat = FileFormat {
    id: 110_226_429,
    source_type: SourceType::Wikidata,
    name: "ELAN Preference File",
    extensions: &["pfsx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
