use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27967101: FileFormat = FileFormat {
    id: 27_967_101,
    puid: "wikidata/27967101",
    name: "Nintendo GameCube/Wii ADP",
    extensions: &["adp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
