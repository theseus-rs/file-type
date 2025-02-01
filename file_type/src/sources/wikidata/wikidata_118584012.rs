use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_118584012: FileFormat = FileFormat {
    id: 118_584_012,
    puid: "wikidata/118584012",
    name: "Cakewalk Template",
    extensions: &["cwt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
