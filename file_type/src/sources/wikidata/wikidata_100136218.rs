use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_100136218: FileFormat = FileFormat {
    id: 100_136_218,
    source_type: SourceType::Wikidata,
    name: "XDOMEA 2.2.0",
    extensions: &["xml"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
