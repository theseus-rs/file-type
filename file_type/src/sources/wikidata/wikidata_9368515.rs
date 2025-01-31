use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_9368515: FileFormat = FileFormat {
    id: 9_368_515,
    puid: "wikidata/9368515",
    name: "MFS",
    extensions: &["mfs"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
