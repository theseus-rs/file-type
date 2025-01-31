use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_33515561: FileFormat = FileFormat {
    id: 33_515_561,
    puid: "wikidata/33515561",
    name: "LAS 1.3 file format",
    extensions: &["las", "laz"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
