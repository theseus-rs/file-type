use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28205468: FileFormat = FileFormat {
    id: 28_205_468,
    puid: "wikidata/28205468",
    name: "Sony Mavica 411",
    extensions: &["411"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
