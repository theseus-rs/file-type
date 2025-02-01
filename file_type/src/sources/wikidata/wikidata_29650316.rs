use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29650316: FileFormat = FileFormat {
    id: 29_650_316,
    puid: "wikidata/29650316",
    name: "Packed Font File Format",
    extensions: &["pk"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
