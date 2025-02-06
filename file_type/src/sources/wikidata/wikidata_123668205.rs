use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_123668205: FileFormat = FileFormat {
    id: 123_668_205,
    source_type: SourceType::Wikidata,
    name: "LiveCode Stack v5.5",
    extensions: &["livecode", "rev"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
