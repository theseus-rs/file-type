use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_131012500: FileFormat = FileFormat {
    id: 131_012_500,
    puid: "wikidata/131012500",
    name: "Stringified NBT format",
    extensions: &["snbt"],
    media_types: &["text/snbt"],
    internal_signatures: &[],
    related_formats: &[],
};
