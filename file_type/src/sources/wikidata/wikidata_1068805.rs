use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_1068805: FileFormat = FileFormat {
    id: 1_068_805,
    puid: "wikidata/1068805",
    name: ".properties",
    extensions: &["properties"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
