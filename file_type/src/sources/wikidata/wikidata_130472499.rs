use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_130472499: FileFormat = FileFormat {
    id: 130_472_499,
    puid: "wikidata/130472499",
    name: "Pig source code file",
    extensions: &["pig"],
    media_types: &["text/x-pig"],
    internal_signatures: &[],
    related_formats: &[],
};
