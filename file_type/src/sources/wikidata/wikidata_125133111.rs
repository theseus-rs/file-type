use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_125133111: FileFormat = FileFormat {
    id: 125_133_111,
    puid: "wikidata/125133111",
    name: "ArcGIS Pro Project file",
    extensions: &["aprx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
