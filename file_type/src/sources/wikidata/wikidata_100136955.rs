use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_100136955: FileFormat = FileFormat {
    id: 100_136_955,
    source_type: SourceType::Wikidata,
    name: "XDOMEA 2.3.0",
    extensions: &["xml"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
