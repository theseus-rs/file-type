use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_118145066: FileFormat = FileFormat {
    id: 118_145_066,
    source_type: SourceType::Wikidata,
    name: "Serenade Harmonica File",
    extensions: &["ckt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
