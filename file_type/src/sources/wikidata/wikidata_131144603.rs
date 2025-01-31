use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_131144603: FileFormat = FileFormat {
    id: 131_144_603,
    puid: "wikidata/131144603",
    name: "Sophia file format",
    extensions: &["aes"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
