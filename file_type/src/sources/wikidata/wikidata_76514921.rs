use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_76514921: FileFormat = FileFormat {
    id: 76_514_921,
    puid: "wikidata/76514921",
    name: "WinDev Window",
    extensions: &["wdw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
