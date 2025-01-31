use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_112960222: FileFormat = FileFormat {
    id: 112_960_222,
    puid: "wikidata/112960222",
    name: "GameExchange2 camera file",
    extensions: &["gcf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
