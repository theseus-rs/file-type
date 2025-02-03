use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_120079718: FileFormat = FileFormat {
    id: 120_079_718,
    source_type: SourceType::Wikidata,
    name: "Matisse file",
    extensions: &["mat"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
