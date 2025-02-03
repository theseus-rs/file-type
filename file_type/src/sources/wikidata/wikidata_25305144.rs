use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_25305144: FileFormat = FileFormat {
    id: 25_305_144,
    source_type: SourceType::Wikidata,
    name: "ShrinkIt",
    extensions: &["shk"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
