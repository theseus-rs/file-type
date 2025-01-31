use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29650304: FileFormat = FileFormat {
    id: 29_650_304,
    puid: "wikidata/29650304",
    name: "PRT scene description",
    extensions: &["dat"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
