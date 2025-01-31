use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_125868433: FileFormat = FileFormat {
    id: 125_868_433,
    puid: "wikidata/125868433",
    name: "OpenWayback CDXJ File Format",
    extensions: &["cdx", "cdxj"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
