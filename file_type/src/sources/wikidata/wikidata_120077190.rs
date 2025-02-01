use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_120077190: FileFormat = FileFormat {
    id: 120_077_190,
    puid: "wikidata/120077190",
    name: "ElectricImage IMAGE",
    extensions: &["ei", "img"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
