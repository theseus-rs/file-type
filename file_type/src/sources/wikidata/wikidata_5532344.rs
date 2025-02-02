use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_5532344: FileFormat = FileFormat {
    id: 5_532_344,
    source_type: SourceType::Wikidata,
    name: "General feature format",
    extensions: &["gff"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
