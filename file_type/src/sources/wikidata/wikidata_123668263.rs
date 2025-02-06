use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_123668263: FileFormat = FileFormat {
    id: 123_668_263,
    source_type: SourceType::Wikidata,
    name: "LiveCode Stack 7.0",
    extensions: &["livecode", "rev"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
