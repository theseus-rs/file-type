use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_120079718: FileFormat = FileFormat {
    id: 120_079_718,
    source_type: SourceType::Wikidata,
    name: "Matisse file",
    extensions: &["mat"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
