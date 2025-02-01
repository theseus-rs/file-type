use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_120805201: FileFormat = FileFormat {
    id: 120_805_201,
    puid: "wikidata/120805201",
    name: "OCP Art Studio Screen File",
    extensions: &["SCR"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
