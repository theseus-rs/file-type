use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_129485975: FileFormat = FileFormat {
    id: 129_485_975,
    source_type: SourceType::Wikidata,
    name: "GraphQL file format",
    extensions: &["graphql"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
