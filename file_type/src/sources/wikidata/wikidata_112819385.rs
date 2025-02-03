use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_112819385: FileFormat = FileFormat {
    id: 112_819_385,
    source_type: SourceType::Wikidata,
    name: "Alias TRIangle file",
    extensions: &["tri"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
