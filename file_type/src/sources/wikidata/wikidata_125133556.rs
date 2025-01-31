use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_125133556: FileFormat = FileFormat {
    id: 125_133_556,
    puid: "wikidata/125133556",
    name: "ArcGIS Pro Map package",
    extensions: &["mpkx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
