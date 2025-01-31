use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28207058: FileFormat = FileFormat {
    id: 28_207_058,
    puid: "wikidata/28207058",
    name: "Poser Bump Map",
    extensions: &["bum"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
