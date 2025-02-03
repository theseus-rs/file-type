use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_123593968: FileFormat = FileFormat {
    id: 123_593_968,
    source_type: SourceType::Wikidata,
    name: "SGI Movie File",
    extensions: &["movie", "mv"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
