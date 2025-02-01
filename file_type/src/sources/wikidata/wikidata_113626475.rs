use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_113626475: FileFormat = FileFormat {
    id: 113_626_475,
    puid: "wikidata/113626475",
    name: "FOCUS file",
    extensions: &["fex"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
