use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206078: FileFormat = FileFormat {
    id: 28_206_078,
    source_type: SourceType::Wikidata,
    name: "View ST/TT TT-Medium",
    extensions: &["PI5"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
