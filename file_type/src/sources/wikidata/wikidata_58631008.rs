use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_58631008: FileFormat = FileFormat {
    id: 58_631_008,
    puid: "wikidata/58631008",
    name: "Harris Matrix",
    extensions: &["hm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
