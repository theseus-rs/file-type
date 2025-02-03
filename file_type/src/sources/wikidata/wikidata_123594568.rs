use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_123594568: FileFormat = FileFormat {
    id: 123_594_568,
    source_type: SourceType::Wikidata,
    name: "Portable Document Format/Archive, version 4",
    extensions: &["pdf"],
    media_types: &["application/pdf"],
    internal_signatures: &[],
    related_formats: &[],
};
