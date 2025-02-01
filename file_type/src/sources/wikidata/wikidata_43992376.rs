use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_43992376: FileFormat = FileFormat {
    id: 43_992_376,
    puid: "wikidata/43992376",
    name: "ABIF file format",
    extensions: &["abif"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
