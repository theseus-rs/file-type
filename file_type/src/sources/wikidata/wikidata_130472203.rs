use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_130472203: FileFormat = FileFormat {
    id: 130_472_203,
    puid: "wikidata/130472203",
    name: "Phix file",
    extensions: &["exw"],
    media_types: &["text/x-phix"],
    internal_signatures: &[],
    related_formats: &[],
};
