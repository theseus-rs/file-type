use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_48693254: FileFormat = FileFormat {
    id: 48_693_254,
    puid: "wikidata/48693254",
    name: "WordStar for MSDOS Document, version 4",
    extensions: &["ws", "ws4"],
    media_types: &["application/x-wordstar", "application/x-wordstar"],
    internal_signatures: &[],
    related_formats: &[],
};
