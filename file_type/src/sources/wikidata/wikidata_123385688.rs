use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_123385688: FileFormat = FileFormat {
    id: 123_385_688,
    source_type: SourceType::Wikidata,
    name: "iSpace 1.0 Scene file",
    extensions: &["iss"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
