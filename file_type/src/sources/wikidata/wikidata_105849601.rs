use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105849601: FileFormat = FileFormat {
    id: 105_849_601,
    source_type: SourceType::Wikidata,
    name: "Help File Contents",
    extensions: &["cnt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
