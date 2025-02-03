use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855190: FileFormat = FileFormat {
    id: 105_855_190,
    source_type: SourceType::Wikidata,
    name: "Fractal Flame Parameters",
    extensions: &["flame"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
