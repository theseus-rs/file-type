use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_116648213: FileFormat = FileFormat {
    id: 116_648_213,
    puid: "wikidata/116648213",
    name: "Template file",
    extensions: &["ofl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
