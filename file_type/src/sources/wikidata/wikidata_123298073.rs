use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_123298073: FileFormat = FileFormat {
    id: 123_298_073,
    puid: "wikidata/123298073",
    name: "Lotus Organizer 2.x/97 mapping",
    extensions: &["csv"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
