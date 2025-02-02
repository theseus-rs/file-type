use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_100136218: FileFormat = FileFormat {
    id: 100_136_218,
    source_type: SourceType::Wikidata,
    name: "XDOMEA 2.2.0",
    extensions: &["xml"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
