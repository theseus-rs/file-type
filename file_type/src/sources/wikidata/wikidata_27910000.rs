use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27910000: FileFormat = FileFormat {
    id: 27_910_000,
    puid: "wikidata/27910000",
    name: "RADARSAT-2 Product Information File",
    extensions: &["xml"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
