use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_76515169: FileFormat = FileFormat {
    id: 76_515_169,
    puid: "wikidata/76515169",
    name: "Windows Runtime Metadata",
    extensions: &["winmd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
