use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105862930: FileFormat = FileFormat {
    id: 105_862_930,
    puid: "wikidata/105862930",
    name: "Aleph One Marathon Markup Language",
    extensions: &["mml"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
