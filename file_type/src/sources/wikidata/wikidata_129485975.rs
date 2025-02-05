use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_129485975: FileFormat = FileFormat {
    id: 129_485_975,
    source_type: SourceType::Wikidata,
    name: "GraphQL file format",
    extensions: &["graphql"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
