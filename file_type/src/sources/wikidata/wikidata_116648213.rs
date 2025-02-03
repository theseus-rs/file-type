use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_116648213: FileFormat = FileFormat {
    id: 116_648_213,
    source_type: SourceType::Wikidata,
    name: "Template file",
    extensions: &["ofl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
