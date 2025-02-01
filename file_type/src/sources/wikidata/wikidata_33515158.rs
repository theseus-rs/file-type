use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_33515158: FileFormat = FileFormat {
    id: 33_515_158,
    puid: "wikidata/33515158",
    name: "LAS 1.1",
    extensions: &["las", "laz"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
