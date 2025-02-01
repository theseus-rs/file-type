use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_1075962: FileFormat = FileFormat {
    id: 1_075_962,
    puid: "wikidata/1075962",
    name: "RealMedia",
    extensions: &["rm", "rv"],
    media_types: &[
        "application/vnd.rn-realmedia",
        "application/vnd.rn-realmedia",
    ],
    internal_signatures: &[],
    related_formats: &[],
};
