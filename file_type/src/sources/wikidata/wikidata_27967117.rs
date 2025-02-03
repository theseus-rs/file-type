use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27967117: FileFormat = FileFormat {
    id: 27_967_117,
    source_type: SourceType::Wikidata,
    name: "B's Pro Tracker module",
    extensions: &["bpm", "bps"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
