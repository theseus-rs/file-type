use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_120762745: FileFormat = FileFormat {
    id: 120_762_745,
    puid: "wikidata/120762745",
    name: "Topo USA 4.0 File",
    extensions: &["tp4"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
