use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_81526237: FileFormat = FileFormat {
    id: 81_526_237,
    puid: "wikidata/81526237",
    name: "MapInfo MapBasic tabular DataBase",
    extensions: &["dat"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
