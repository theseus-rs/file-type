use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_110239092: FileFormat = FileFormat {
    id: 110_239_092,
    puid: "wikidata/110239092",
    name: "Avid Editor Format",
    extensions: &["txt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
