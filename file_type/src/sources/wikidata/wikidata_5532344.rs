use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_5532344: FileFormat = FileFormat {
    id: 5_532_344,
    source_type: SourceType::Wikidata,
    name: "General feature format",
    extensions: &["gff"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
