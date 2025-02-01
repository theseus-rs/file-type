use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_125871478: FileFormat = FileFormat {
    id: 125_871_478,
    puid: "wikidata/125871478",
    name: "PechaMaker Format",
    extensions: &["pxp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
