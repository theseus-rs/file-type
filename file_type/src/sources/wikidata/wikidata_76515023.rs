use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_76515023: FileFormat = FileFormat {
    id: 76_515_023,
    puid: "wikidata/76515023",
    name: "Safari Web History",
    extensions: &["webhistory"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
