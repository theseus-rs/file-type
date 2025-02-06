use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_130459044: FileFormat = FileFormat {
    id: 130_459_044,
    source_type: SourceType::Wikidata,
    name: "Pawn source code file",
    extensions: &["p", "pwn"],
    media_types: &["text/x-pawn"],
    signatures: &[],
    related_formats: &[],
};
