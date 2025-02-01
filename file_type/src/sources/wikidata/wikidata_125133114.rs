use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_125133114: FileFormat = FileFormat {
    id: 125_133_114,
    puid: "wikidata/125133114",
    name: "ArcGIS Pro Project Package",
    extensions: &["ppkx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
