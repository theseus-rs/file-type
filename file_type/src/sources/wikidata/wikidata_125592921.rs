use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_125592921: FileFormat = FileFormat {
    id: 125_592_921,
    puid: "wikidata/125592921",
    name: "Raw CMYK",
    extensions: &["cmyk"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
