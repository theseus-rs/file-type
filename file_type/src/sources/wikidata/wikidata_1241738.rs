use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_1241738: FileFormat = FileFormat {
    id: 1_241_738,
    source_type: SourceType::Wikidata,
    name: "M3U",
    extensions: &["m3u8"],
    media_types: &["audio/x-mpegurl"],
    signatures: &[],
    related_formats: &[],
};
