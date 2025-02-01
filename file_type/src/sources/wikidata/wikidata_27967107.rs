use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27967107: FileFormat = FileFormat {
    id: 27_967_107,
    puid: "wikidata/27967107",
    name: "Ragnarok Online 2 RMP",
    extensions: &["rmp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
