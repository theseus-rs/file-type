use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27960004: FileFormat = FileFormat {
    id: 27_960_004,
    puid: "wikidata/27960004",
    name: "Real Lossless Codec",
    extensions: &["rmvb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
