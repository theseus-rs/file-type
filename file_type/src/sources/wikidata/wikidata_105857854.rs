use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857854: FileFormat = FileFormat {
    id: 105_857_854,
    source_type: SourceType::Wikidata,
    name: "Ivy module descriptor",
    extensions: &["ivy"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
