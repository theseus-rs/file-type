use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29650342: FileFormat = FileFormat {
    id: 29_650_342,
    puid: "wikidata/29650342",
    name: "PEM encoded certificate",
    extensions: &["cer", "crt", "pem"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
