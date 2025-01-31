use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_118436204: FileFormat = FileFormat {
    id: 118_436_204,
    puid: "wikidata/118436204",
    name: "Esri ArcMap Label file",
    extensions: &["lxp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
