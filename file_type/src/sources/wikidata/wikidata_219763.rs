use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_219763: FileFormat = FileFormat {
    id: 219_763,
    puid: "wikidata/219763",
    name: "MPEG-4",
    extensions: &["mp4"],
    media_types: &["video/mp4"],
    internal_signatures: &[],
    related_formats: &[],
};
