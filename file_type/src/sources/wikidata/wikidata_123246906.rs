use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_123246906: FileFormat = FileFormat {
    id: 123_246_906,
    source_type: SourceType::Wikidata,
    name: "Movie Collector Database",
    extensions: &["mvc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
