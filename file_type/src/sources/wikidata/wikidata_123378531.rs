use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_123378531: FileFormat = FileFormat {
    id: 123_378_531,
    source_type: SourceType::Wikidata,
    name: "Curve library",
    extensions: &["cvl"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
