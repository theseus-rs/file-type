use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_100323905: FileFormat = FileFormat {
    id: 100_323_905,
    puid: "wikidata/100323905",
    name: "PFS:Write Document",
    extensions: &["pfs"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
