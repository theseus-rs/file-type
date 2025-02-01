use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_122509776: FileFormat = FileFormat {
    id: 122_509_776,
    puid: "wikidata/122509776",
    name: "Pretty Good Privacy public key ring data file",
    extensions: &["pubkr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
