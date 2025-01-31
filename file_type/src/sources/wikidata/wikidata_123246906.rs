use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_123246906: FileFormat = FileFormat {
    id: 123_246_906,
    puid: "wikidata/123246906",
    name: "Movie Collector Database",
    extensions: &["mvc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
