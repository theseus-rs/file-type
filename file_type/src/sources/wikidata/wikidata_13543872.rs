use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_13543872: FileFormat = FileFormat {
    id: 13_543_872,
    puid: "wikidata/13543872",
    name: "Wii ISO Archive",
    extensions: &["wbfs"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
