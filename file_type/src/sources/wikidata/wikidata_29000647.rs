use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29000647: FileFormat = FileFormat {
    id: 29_000_647,
    source_type: SourceType::Wikidata,
    name: "PLG",
    extensions: &["plg"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
