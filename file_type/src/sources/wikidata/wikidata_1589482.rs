use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_1589482: FileFormat = FileFormat {
    id: 1_589_482,
    puid: "wikidata/1589482",
    name: "JT",
    extensions: &["JT"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
