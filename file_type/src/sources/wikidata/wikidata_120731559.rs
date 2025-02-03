use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_120731559: FileFormat = FileFormat {
    id: 120_731_559,
    source_type: SourceType::Wikidata,
    name: "MotionMaker Template",
    extensions: &["fmt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
