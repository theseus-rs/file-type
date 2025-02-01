use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_114079055: FileFormat = FileFormat {
    id: 114_079_055,
    puid: "wikidata/114079055",
    name: "MacBinary III",
    extensions: &["bin"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
