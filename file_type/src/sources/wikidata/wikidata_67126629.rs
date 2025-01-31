use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_67126629: FileFormat = FileFormat {
    id: 67_126_629,
    puid: "wikidata/67126629",
    name: "Print Artist sign file format",
    extensions: &["sgn"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
