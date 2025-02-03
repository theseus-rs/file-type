use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_361923: FileFormat = FileFormat {
    id: 361_923,
    source_type: SourceType::Wikidata,
    name: "Lossless predictive audio compression",
    extensions: &["pac"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
