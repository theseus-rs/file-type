use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27824056: FileFormat = FileFormat {
    id: 27_824_056,
    puid: "wikidata/27824056",
    name: "ar, Coherent variant",
    extensions: &["a"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
