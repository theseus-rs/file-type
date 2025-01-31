use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111292287: FileFormat = FileFormat {
    id: 111_292_287,
    puid: "wikidata/111292287",
    name: "linux shared library",
    extensions: &["so"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
