use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27486884: FileFormat = FileFormat {
    id: 27_486_884,
    puid: "wikidata/27486884",
    name: "Shapefile main file",
    extensions: &["shp"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
