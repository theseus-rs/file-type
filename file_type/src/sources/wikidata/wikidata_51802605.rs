use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_51802605: FileFormat = FileFormat {
    id: 51_802_605,
    puid: "wikidata/51802605",
    name: "OS/2 Change Control File",
    extensions: &["cin"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
