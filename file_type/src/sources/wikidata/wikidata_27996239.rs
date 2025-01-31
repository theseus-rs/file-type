use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27996239: FileFormat = FileFormat {
    id: 27_996_239,
    puid: "wikidata/27996239",
    name: "Faster than Light saved game",
    extensions: &["sav"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
