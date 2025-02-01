use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_4227994: FileFormat = FileFormat {
    id: 4_227_994,
    puid: "wikidata/4227994",
    name: "Shareaza collection",
    extensions: &["co"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
