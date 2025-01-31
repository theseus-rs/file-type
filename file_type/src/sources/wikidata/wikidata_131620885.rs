use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_131620885: FileFormat = FileFormat {
    id: 131_620_885,
    puid: "wikidata/131620885",
    name: "MotionFX motion definition file",
    extensions: &["cfg"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
