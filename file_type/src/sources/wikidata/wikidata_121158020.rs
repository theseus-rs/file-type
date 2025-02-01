use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_121158020: FileFormat = FileFormat {
    id: 121_158_020,
    puid: "wikidata/121158020",
    name: "Letter file",
    extensions: &["rtf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
