use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28207044: FileFormat = FileFormat {
    id: 28_207_044,
    source_type: SourceType::Wikidata,
    name: "PM",
    extensions: &["pm"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
