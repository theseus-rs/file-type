use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28206404: FileFormat = FileFormat {
    id: 28_206_404,
    source_type: SourceType::Wikidata,
    name: "JEDMICS C4",
    extensions: &["c4", "ct4"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
