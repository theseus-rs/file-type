use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_48915661: FileFormat = FileFormat {
    id: 48_915_661,
    puid: "wikidata/48915661",
    name: "Interleaf Document",
    extensions: &["doc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
