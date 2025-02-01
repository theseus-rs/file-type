use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27979377: FileFormat = FileFormat {
    id: 27_979_377,
    puid: "wikidata/27979377",
    name: "VobSub subtitle",
    extensions: &["sub"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
