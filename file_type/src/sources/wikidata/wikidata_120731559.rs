use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_120731559: FileFormat = FileFormat {
    id: 120_731_559,
    puid: "wikidata/120731559",
    name: "MotionMaker Template",
    extensions: &["fmt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
