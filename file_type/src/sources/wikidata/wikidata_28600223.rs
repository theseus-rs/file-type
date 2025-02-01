use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28600223: FileFormat = FileFormat {
    id: 28_600_223,
    puid: "wikidata/28600223",
    name: "APT",
    extensions: &["apt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
