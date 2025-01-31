use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_11224899: FileFormat = FileFormat {
    id: 11_224_899,
    puid: "wikidata/11224899",
    name: "ish",
    extensions: &["ish"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
