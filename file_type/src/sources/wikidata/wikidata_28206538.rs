use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206538: FileFormat = FileFormat {
    id: 28_206_538,
    puid: "wikidata/28206538",
    name: "Magick Persistent Cache (.cache file)",
    extensions: &["cache"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
