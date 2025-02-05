use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_58077776: FileFormat = FileFormat {
    id: 58_077_776,
    source_type: SourceType::Wikidata,
    name: "AbiWord Document Template",
    extensions: &["awt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
