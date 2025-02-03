use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_123668527: FileFormat = FileFormat {
    id: 123_668_527,
    source_type: SourceType::Wikidata,
    name: "LiveCode Stack 8.1+",
    extensions: &["livecode", "rev"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
