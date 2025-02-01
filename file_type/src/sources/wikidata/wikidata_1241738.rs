use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_1241738: FileFormat = FileFormat {
    id: 1_241_738,
    puid: "wikidata/1241738",
    name: "M3U",
    extensions: &["m3u8"],
    media_types: &["audio/x-mpegurl"],
    internal_signatures: &[],
    related_formats: &[],
};
