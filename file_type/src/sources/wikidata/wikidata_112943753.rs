use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_112943753: FileFormat = FileFormat {
    id: 112_943_753,
    puid: "wikidata/112943753",
    name: "GameExchange2 GRP file",
    extensions: &["grp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
