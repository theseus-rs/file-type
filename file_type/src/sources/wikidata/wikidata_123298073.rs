use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_123298073: FileFormat = FileFormat {
    id: 123_298_073,
    source_type: SourceType::Wikidata,
    name: "Lotus Organizer 2.x/97 mapping",
    extensions: &["csv"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
