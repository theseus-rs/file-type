use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111355364: FileFormat = FileFormat {
    id: 111_355_364,
    puid: "wikidata/111355364",
    name: "Covox 8-bit audio",
    extensions: &["v8"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
