use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27967124: FileFormat = FileFormat {
    id: 27_967_124,
    puid: "wikidata/27967124",
    name: "CM3",
    extensions: &["cm3"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
