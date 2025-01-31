use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_1166919: FileFormat = FileFormat {
    id: 1_166_919,
    puid: "wikidata/1166919",
    name: "Darwin Information Typing Architecture",
    extensions: &["dita", "xml"],
    media_types: &["application/dita+xml", "application/dita+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
