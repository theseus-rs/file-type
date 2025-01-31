use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_120763372: FileFormat = FileFormat {
    id: 120_763_372,
    puid: "wikidata/120763372",
    name: "Topo USA 3.0 File",
    extensions: &["tp3"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
