use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_125868433: FileFormat = FileFormat {
    id: 125_868_433,
    source_type: SourceType::Wikidata,
    name: "OpenWayback CDXJ File Format",
    extensions: &["cdx", "cdxj"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
