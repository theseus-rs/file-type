use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857324: FileFormat = FileFormat {
    id: 105_857_324,
    puid: "wikidata/105857324",
    name: "JAXB Bindings",
    extensions: &["jxb", "xjb"],
    media_types: &["application/octet-stream", "application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
