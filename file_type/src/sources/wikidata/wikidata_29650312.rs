use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29650312: FileFormat = FileFormat {
    id: 29_650_312,
    puid: "wikidata/29650312",
    name: "PMA",
    extensions: &["pma"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
