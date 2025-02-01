use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29000561: FileFormat = FileFormat {
    id: 29_000_561,
    puid: "wikidata/29000561",
    name: "Kryoflux Stream",
    extensions: &["raw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
