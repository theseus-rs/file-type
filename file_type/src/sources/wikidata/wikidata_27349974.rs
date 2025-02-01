use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27349974: FileFormat = FileFormat {
    id: 27_349_974,
    puid: "wikidata/27349974",
    name: "TOPSAR Incidence Angle Map",
    extensions: &["incgr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
