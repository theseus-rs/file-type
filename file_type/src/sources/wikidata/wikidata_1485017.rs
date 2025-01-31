use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_1485017: FileFormat = FileFormat {
    id: 1_485_017,
    puid: "wikidata/1485017",
    name: "GDSII stream format",
    extensions: &["gds"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
