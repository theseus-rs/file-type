use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_100136955: FileFormat = FileFormat {
    id: 100_136_955,
    source_type: SourceType::Wikidata,
    name: "XDOMEA 2.3.0",
    extensions: &["xml"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
