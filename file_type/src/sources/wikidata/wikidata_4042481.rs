use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_4042481: FileFormat = FileFormat {
    id: 4_042_481,
    puid: "wikidata/4042481",
    name: "LOGML",
    extensions: &["xml"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
