use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_919226: FileFormat = FileFormat {
    id: 919_226,
    puid: "wikidata/919226",
    name: "MPEG-1 Audio Layer I",
    extensions: &["m1a", "m1a", "mp1", "mp1"],
    media_types: &["audio/MPA", "audio/MPA", "audio/mpeg", "audio/mpeg"],
    internal_signatures: &[],
    related_formats: &[],
};
