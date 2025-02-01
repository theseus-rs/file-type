use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_112943858: FileFormat = FileFormat {
    id: 112_943_858,
    puid: "wikidata/112943858",
    name: "GameExchange2 material definition file",
    extensions: &["gmf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
