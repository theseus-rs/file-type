use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_109334805: FileFormat = FileFormat {
    id: 109_334_805,
    puid: "wikidata/109334805",
    name: "Advanced Comic Book Format",
    extensions: &["acbf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
