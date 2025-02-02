use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_123595865: FileFormat = FileFormat {
    id: 123_595_865,
    source_type: SourceType::Wikidata,
    name: "Portable Document Format/Archive, version 4f",
    extensions: &["pdf"],
    media_types: &["application/pdf"],
    internal_signatures: &[],
    related_formats: &[],
};
