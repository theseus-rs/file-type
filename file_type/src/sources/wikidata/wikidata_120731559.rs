use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_120731559: FileFormat = FileFormat {
    id: 120_731_559,
    source_type: SourceType::Wikidata,
    name: "MotionMaker Template",
    extensions: &["fmt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
