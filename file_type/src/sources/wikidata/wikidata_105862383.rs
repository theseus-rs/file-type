use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105862383: FileFormat = FileFormat {
    id: 105_862_383,
    puid: "wikidata/105862383",
    name: "Minecraft pack info",
    extensions: &["mcmeta"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
