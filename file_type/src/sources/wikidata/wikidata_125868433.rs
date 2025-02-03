use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_125868433: FileFormat = FileFormat {
    id: 125_868_433,
    source_type: SourceType::Wikidata,
    name: "OpenWayback CDXJ File Format",
    extensions: &["cdx", "cdxj"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
