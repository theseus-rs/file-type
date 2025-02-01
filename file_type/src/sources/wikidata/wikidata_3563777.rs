use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_3563777: FileFormat = FileFormat {
    id: 3_563_777,
    puid: "wikidata/3563777",
    name: "MicroDVD",
    extensions: &["sub"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
