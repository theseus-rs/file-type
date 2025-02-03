use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_58077776: FileFormat = FileFormat {
    id: 58_077_776,
    source_type: SourceType::Wikidata,
    name: "AbiWord Document Template",
    extensions: &["awt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
