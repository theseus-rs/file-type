use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27967143: FileFormat = FileFormat {
    id: 27_967_143,
    puid: "wikidata/27967143",
    name: "DigiTrekker module",
    extensions: &["dtm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
