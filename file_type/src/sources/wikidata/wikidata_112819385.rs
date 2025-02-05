use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_112819385: FileFormat = FileFormat {
    id: 112_819_385,
    source_type: SourceType::Wikidata,
    name: "Alias TRIangle file",
    extensions: &["tri"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
