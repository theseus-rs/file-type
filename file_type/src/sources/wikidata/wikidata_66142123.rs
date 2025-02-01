use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_66142123: FileFormat = FileFormat {
    id: 66_142_123,
    puid: "wikidata/66142123",
    name: "ACCDE file format",
    extensions: &["accde"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
