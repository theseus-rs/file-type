use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_131620885: FileFormat = FileFormat {
    id: 131_620_885,
    source_type: SourceType::Wikidata,
    name: "MotionFX motion definition file",
    extensions: &["cfg"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
