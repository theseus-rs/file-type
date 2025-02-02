use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_33514773: FileFormat = FileFormat {
    id: 33_514_773,
    source_type: SourceType::Wikidata,
    name: "LAS 1.0",
    extensions: &["las", "laz"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
