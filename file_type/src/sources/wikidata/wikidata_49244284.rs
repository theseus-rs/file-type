use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_49244284: FileFormat = FileFormat {
    id: 49_244_284,
    puid: "wikidata/49244284",
    name: "form*Z Project File",
    extensions: &["fmz"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
