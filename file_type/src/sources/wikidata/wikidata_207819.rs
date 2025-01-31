use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_207819: FileFormat = FileFormat {
    id: 207_819,
    puid: "wikidata/207819",
    name: "Standard Generalized Markup Language",
    extensions: &["sgml", "sgml"],
    media_types: &["application/sgml", "text/sgml"],
    internal_signatures: &[],
    related_formats: &[],
};
