use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_17072901: FileFormat = FileFormat {
    id: 17_072_901,
    puid: "wikidata/17072901",
    name: "Open Game Engine Exchange",
    extensions: &["ogex"],
    media_types: &["model/vnd.opengex"],
    internal_signatures: &[],
    related_formats: &[],
};
