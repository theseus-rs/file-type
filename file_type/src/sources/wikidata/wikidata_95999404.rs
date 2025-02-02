use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_95999404: FileFormat = FileFormat {
    id: 95_999_404,
    source_type: SourceType::Wikidata,
    name: "Graph6",
    extensions: &["g6"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
