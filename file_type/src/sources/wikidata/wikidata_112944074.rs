use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_112944074: FileFormat = FileFormat {
    id: 112_944_074,
    source_type: SourceType::Wikidata,
    name: "GameExchange2 skeleton file",
    extensions: &["GSF"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
