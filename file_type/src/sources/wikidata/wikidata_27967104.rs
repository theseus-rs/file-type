use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27967104: FileFormat = FileFormat {
    id: 27_967_104,
    puid: "wikidata/27967104",
    name: "Shroom Instrument",
    extensions: &["shi"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
