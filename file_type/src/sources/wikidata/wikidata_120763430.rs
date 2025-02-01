use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_120763430: FileFormat = FileFormat {
    id: 120_763_430,
    puid: "wikidata/120763430",
    name: "Topo USA 2.0 File",
    extensions: &["tp2"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
