use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_131287731: FileFormat = FileFormat {
    id: 131_287_731,
    puid: "wikidata/131287731",
    name: "Tea Template file format",
    extensions: &["tea"],
    media_types: &["text/x-tea"],
    internal_signatures: &[],
    related_formats: &[],
};
