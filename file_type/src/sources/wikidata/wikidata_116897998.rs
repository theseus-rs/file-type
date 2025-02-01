use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_116897998: FileFormat = FileFormat {
    id: 116_897_998,
    puid: "wikidata/116897998",
    name: "Minecraft resource pack",
    extensions: &["zip"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
