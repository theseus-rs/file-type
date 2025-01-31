use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_112943826: FileFormat = FileFormat {
    id: 112_943_826,
    puid: "wikidata/112943826",
    name: "GameExchange2 raw object 'body' definition file",
    extensions: &["gbf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
