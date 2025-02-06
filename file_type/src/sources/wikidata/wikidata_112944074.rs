use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_112944074: FileFormat = FileFormat {
    id: 112_944_074,
    source_type: SourceType::Wikidata,
    name: "GameExchange2 skeleton file",
    extensions: &["GSF"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
