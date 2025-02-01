use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_85415600: FileFormat = FileFormat {
    id: 85_415_600,
    puid: "wikidata/85415600",
    name: "Tweet JSON",
    extensions: &["json"],
    media_types: &["application/json"],
    internal_signatures: &[],
    related_formats: &[],
};
