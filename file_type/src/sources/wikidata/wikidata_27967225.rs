use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27967225: FileFormat = FileFormat {
    id: 27_967_225,
    puid: "wikidata/27967225",
    name: "D-Lusion Music File",
    extensions: &["dmf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
