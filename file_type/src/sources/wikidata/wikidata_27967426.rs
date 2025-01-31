use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27967426: FileFormat = FileFormat {
    id: 27_967_426,
    puid: "wikidata/27967426",
    name: "Creative Music System",
    extensions: &["cms"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
