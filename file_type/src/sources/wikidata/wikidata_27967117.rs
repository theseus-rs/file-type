use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27967117: FileFormat = FileFormat {
    id: 27_967_117,
    puid: "wikidata/27967117",
    name: "B's Pro Tracker module",
    extensions: &["bpm", "bps"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
