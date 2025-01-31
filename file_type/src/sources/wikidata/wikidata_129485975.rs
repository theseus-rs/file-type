use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_129485975: FileFormat = FileFormat {
    id: 129_485_975,
    puid: "wikidata/129485975",
    name: "GraphQL file format",
    extensions: &["graphql"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
