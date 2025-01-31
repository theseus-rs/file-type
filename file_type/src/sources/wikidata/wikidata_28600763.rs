use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28600763: FileFormat = FileFormat {
    id: 28_600_763,
    puid: "wikidata/28600763",
    name: "Electronic Arts INF",
    extensions: &["inf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
