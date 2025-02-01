use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27967146: FileFormat = FileFormat {
    id: 27_967_146,
    puid: "wikidata/27967146",
    name: "Eureka Packer module",
    extensions: &["eu"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
