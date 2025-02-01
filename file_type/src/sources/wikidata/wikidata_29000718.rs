use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29000718: FileFormat = FileFormat {
    id: 29_000_718,
    puid: "wikidata/29000718",
    name: "Unreal model aniv file",
    extensions: &["3d"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
