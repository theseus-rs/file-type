use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_49243488: FileFormat = FileFormat {
    id: 49_243_488,
    puid: "wikidata/49243488",
    name: "License file",
    extensions: &["lic"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
