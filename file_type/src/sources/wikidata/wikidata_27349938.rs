use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27349938: FileFormat = FileFormat {
    id: 27_349_938,
    puid: "wikidata/27349938",
    name: "TOPSAR Digital Elevation Model",
    extensions: &["demi2"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
