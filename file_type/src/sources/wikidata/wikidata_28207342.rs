use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28207342: FileFormat = FileFormat {
    id: 28_207_342,
    source_type: SourceType::Wikidata,
    name: "Synu",
    extensions: &["syn", "synu"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
