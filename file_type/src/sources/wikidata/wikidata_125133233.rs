use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_125133233: FileFormat = FileFormat {
    id: 125_133_233,
    puid: "wikidata/125133233",
    name: "ArcGIS Pro Layout file",
    extensions: &["pagx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
