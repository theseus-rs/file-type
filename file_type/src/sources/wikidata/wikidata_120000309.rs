use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_120000309: FileFormat = FileFormat {
    id: 120_000_309,
    puid: "wikidata/120000309",
    name: "ASAP WordPower Presentation",
    extensions: &["asp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
