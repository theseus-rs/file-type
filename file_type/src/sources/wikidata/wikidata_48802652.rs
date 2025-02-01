use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_48802652: FileFormat = FileFormat {
    id: 48_802_652,
    puid: "wikidata/48802652",
    name: "Aldus Freehand Drawing, version 4",
    extensions: &["fh4"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
