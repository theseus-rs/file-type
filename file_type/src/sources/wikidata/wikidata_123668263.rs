use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_123668263: FileFormat = FileFormat {
    id: 123_668_263,
    source_type: SourceType::Wikidata,
    name: "LiveCode Stack 7.0",
    extensions: &["livecode", "rev"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
