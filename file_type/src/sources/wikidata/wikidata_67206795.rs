use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_67206795: FileFormat = FileFormat {
    id: 67_206_795,
    puid: "wikidata/67206795",
    name: "SmartSketch Drawing",
    extensions: &["ssk"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
