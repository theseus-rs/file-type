use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_33515428: FileFormat = FileFormat {
    id: 33_515_428,
    puid: "wikidata/33515428",
    name: "LAS 1.2 file format",
    extensions: &["las", "laz"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
