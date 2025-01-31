use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_130459044: FileFormat = FileFormat {
    id: 130_459_044,
    puid: "wikidata/130459044",
    name: "Pawn source code file",
    extensions: &["p", "pwn"],
    media_types: &["text/x-pawn", "text/x-pawn"],
    internal_signatures: &[],
    related_formats: &[],
};
