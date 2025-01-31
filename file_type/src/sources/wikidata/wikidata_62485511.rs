use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_62485511: FileFormat = FileFormat {
    id: 62_485_511,
    puid: "wikidata/62485511",
    name: "Navisworks Document, version 2011",
    extensions: &["nwc", "nwd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
