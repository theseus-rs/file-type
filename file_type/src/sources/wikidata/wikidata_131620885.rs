use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_131620885: FileFormat = FileFormat {
    id: 131_620_885,
    source_type: SourceType::Wikidata,
    name: "MotionFX motion definition file",
    extensions: &["cfg"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
