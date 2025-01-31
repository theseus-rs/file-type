use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27967116: FileFormat = FileFormat {
    id: 27_967_116,
    puid: "wikidata/27967116",
    name: "ASC Sound Master module",
    extensions: &["asc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
