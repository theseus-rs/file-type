use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_114877188: FileFormat = FileFormat {
    id: 114_877_188,
    puid: "wikidata/114877188",
    name: "Money 1.0, 2.0, and 3.0 Backup File",
    extensions: &["bak"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
