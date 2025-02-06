use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_67206795: FileFormat = FileFormat {
    id: 67_206_795,
    source_type: SourceType::Wikidata,
    name: "SmartSketch Drawing",
    extensions: &["ssk"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
