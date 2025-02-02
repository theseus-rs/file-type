use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27910000: FileFormat = FileFormat {
    id: 27_910_000,
    source_type: SourceType::Wikidata,
    name: "RADARSAT-2 Product Information File",
    extensions: &["xml"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
