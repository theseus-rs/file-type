use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27967189: FileFormat = FileFormat {
    id: 27_967_189,
    puid: "wikidata/27967189",
    name: "Fuzzac Packer module",
    extensions: &["fuz", "fuzz"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
