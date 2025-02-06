use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_123298073: FileFormat = FileFormat {
    id: 123_298_073,
    source_type: SourceType::Wikidata,
    name: "Lotus Organizer 2.x/97 mapping",
    extensions: &["csv"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
