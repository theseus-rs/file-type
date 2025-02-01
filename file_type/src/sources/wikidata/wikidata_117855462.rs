use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_117855462: FileFormat = FileFormat {
    id: 117_855_462,
    puid: "wikidata/117855462",
    name: "PRODUCT R&D Fax File",
    extensions: &["prd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
