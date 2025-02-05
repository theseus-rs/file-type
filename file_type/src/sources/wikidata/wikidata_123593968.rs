use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_123593968: FileFormat = FileFormat {
    id: 123_593_968,
    source_type: SourceType::Wikidata,
    name: "SGI Movie File",
    extensions: &["movie", "mv"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
