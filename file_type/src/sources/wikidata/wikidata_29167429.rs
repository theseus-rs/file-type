use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29167429: FileFormat = FileFormat {
    id: 29_167_429,
    source_type: SourceType::Wikidata,
    name: "NovaMind",
    extensions: &["nmind"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
