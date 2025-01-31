use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_116250065: FileFormat = FileFormat {
    id: 116_250_065,
    puid: "wikidata/116250065",
    name: "Norton System Doctor configuration file",
    extensions: &["nsd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
