use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_131332032: FileFormat = FileFormat {
    id: 131_332_032,
    puid: "wikidata/131332032",
    name: "TypoScript code",
    extensions: &["typoscript"],
    media_types: &["text/x-typoscript"],
    internal_signatures: &[],
    related_formats: &[],
};
