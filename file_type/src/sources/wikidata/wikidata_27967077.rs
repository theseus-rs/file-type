use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27967077: FileFormat = FileFormat {
    id: 27_967_077,
    puid: "wikidata/27967077",
    name: "Beathoven Synthesiser",
    extensions: &["bss"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
