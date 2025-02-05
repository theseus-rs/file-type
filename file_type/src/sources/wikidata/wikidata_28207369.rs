use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28207369: FileFormat = FileFormat {
    id: 28_207_369,
    source_type: SourceType::Wikidata,
    name: "Technicolor Dream LUM",
    extensions: &["lum"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
