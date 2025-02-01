use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_2328734: FileFormat = FileFormat {
    id: 2_328_734,
    puid: "wikidata/2328734",
    name: "JISP",
    extensions: &["jisp"],
    media_types: &["application/vnd.jisp"],
    internal_signatures: &[],
    related_formats: &[],
};
