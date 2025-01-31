use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_123204255: FileFormat = FileFormat {
    id: 123_204_255,
    puid: "wikidata/123204255",
    name: "Avid Media Composer Script",
    extensions: &["avc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
