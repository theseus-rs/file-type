use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29000578: FileFormat = FileFormat {
    id: 29_000_578,
    source_type: SourceType::Wikidata,
    name: "Android Resource file",
    extensions: &["arsc", "xml"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
