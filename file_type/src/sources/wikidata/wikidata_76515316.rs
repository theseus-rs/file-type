use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_76515316: FileFormat = FileFormat {
    id: 76_515_316,
    puid: "wikidata/76515316",
    name: "MapInfo Workspace",
    extensions: &["wor"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
