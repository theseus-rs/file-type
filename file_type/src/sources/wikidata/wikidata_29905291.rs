use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29905291: FileFormat = FileFormat {
    id: 29_905_291,
    puid: "wikidata/29905291",
    name: "Self-Extracting Archive",
    extensions: &["sfx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
