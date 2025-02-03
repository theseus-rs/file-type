use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28207369: FileFormat = FileFormat {
    id: 28_207_369,
    source_type: SourceType::Wikidata,
    name: "Technicolor Dream LUM",
    extensions: &["lum"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
