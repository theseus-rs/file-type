use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28206078: FileFormat = FileFormat {
    id: 28_206_078,
    source_type: SourceType::Wikidata,
    name: "View ST/TT TT-Medium",
    extensions: &["PI5"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
