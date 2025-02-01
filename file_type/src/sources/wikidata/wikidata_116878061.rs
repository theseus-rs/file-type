use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_116878061: FileFormat = FileFormat {
    id: 116_878_061,
    puid: "wikidata/116878061",
    name: "Calendar Creator CSV Event File",
    extensions: &["csv"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
