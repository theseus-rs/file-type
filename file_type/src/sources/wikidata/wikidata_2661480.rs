use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_2661480: FileFormat = FileFormat {
    id: 2_661_480,
    puid: "wikidata/2661480",
    name: "BSON",
    extensions: &["bson"],
    media_types: &["application/bson"],
    internal_signatures: &[],
    related_formats: &[],
};
