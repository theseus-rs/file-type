use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_130279787: FileFormat = FileFormat {
    id: 130_279_787,
    puid: "wikidata/130279787",
    name: "MAQL script file",
    extensions: &["maql", "maql"],
    media_types: &["application/x-gooddata-maql", "text/x-gooddata-maql"],
    internal_signatures: &[],
    related_formats: &[],
};
