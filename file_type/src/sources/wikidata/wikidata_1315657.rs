use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_1315657: FileFormat = FileFormat {
    id: 1_315_657,
    puid: "wikidata/1315657",
    name: "Textile",
    extensions: &["textile"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
