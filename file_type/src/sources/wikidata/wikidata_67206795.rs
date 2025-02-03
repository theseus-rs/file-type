use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_67206795: FileFormat = FileFormat {
    id: 67_206_795,
    source_type: SourceType::Wikidata,
    name: "SmartSketch Drawing",
    extensions: &["ssk"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
