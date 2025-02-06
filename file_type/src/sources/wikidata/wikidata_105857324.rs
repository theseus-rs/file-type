use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857324: FileFormat = FileFormat {
    id: 105_857_324,
    source_type: SourceType::Wikidata,
    name: "JAXB Bindings",
    extensions: &["jxb", "xjb"],
    media_types: &["application/octet-stream"],
    signatures: &[],
    related_formats: &[],
};
