use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_10394820: FileFormat = FileFormat {
    id: 10_394_820,
    source_type: SourceType::Wikidata,
    name: "Zope Configuration Markup Language",
    extensions: &["zcml"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
