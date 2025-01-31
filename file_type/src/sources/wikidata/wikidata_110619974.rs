use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_110619974: FileFormat = FileFormat {
    id: 110_619_974,
    puid: "wikidata/110619974",
    name: "Adobe Atmosphere world (.atmo)",
    extensions: &["3da", "aer", "atmo"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
