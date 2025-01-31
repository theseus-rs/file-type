use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28207038: FileFormat = FileFormat {
    id: 28_207_038,
    puid: "wikidata/28207038",
    name: "Photo Line Document",
    extensions: &["pld"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
