use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_114050529: FileFormat = FileFormat {
    id: 114_050_529,
    puid: "wikidata/114050529",
    name: "Canon MIF File",
    extensions: &["mif"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
