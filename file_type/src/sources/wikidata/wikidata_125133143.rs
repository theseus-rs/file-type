use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_125133143: FileFormat = FileFormat {
    id: 125_133_143,
    puid: "wikidata/125133143",
    name: "ArcGIS Pro Map file",
    extensions: &["mapx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
