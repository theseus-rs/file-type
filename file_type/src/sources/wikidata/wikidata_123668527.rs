use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_123668527: FileFormat = FileFormat {
    id: 123_668_527,
    source_type: SourceType::Wikidata,
    name: "LiveCode Stack 8.1+",
    extensions: &["livecode", "rev"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
