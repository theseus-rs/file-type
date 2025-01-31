use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_51753051: FileFormat = FileFormat {
    id: 51_753_051,
    puid: "wikidata/51753051",
    name: "3D Studio Shapes",
    extensions: &["shp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
