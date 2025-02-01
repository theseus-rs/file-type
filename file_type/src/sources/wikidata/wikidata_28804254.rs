use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28804254: FileFormat = FileFormat {
    id: 28_804_254,
    puid: "wikidata/28804254",
    name: "Uniform Office Text",
    extensions: &["uot"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
