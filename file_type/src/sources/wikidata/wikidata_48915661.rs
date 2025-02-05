use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_48915661: FileFormat = FileFormat {
    id: 48_915_661,
    source_type: SourceType::Wikidata,
    name: "Interleaf Document",
    extensions: &["doc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
