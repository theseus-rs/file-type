use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27967103: FileFormat = FileFormat {
    id: 27_967_103,
    puid: "wikidata/27967103",
    name: "Nintendo GameCube/Wii BRSTM",
    extensions: &["brstm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
