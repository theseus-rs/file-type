use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_48915661: FileFormat = FileFormat {
    id: 48_915_661,
    source_type: SourceType::Wikidata,
    name: "Interleaf Document",
    extensions: &["doc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
