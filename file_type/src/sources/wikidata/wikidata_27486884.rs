use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27486884: FileFormat = FileFormat {
    id: 27_486_884,
    source_type: SourceType::Wikidata,
    name: "Shapefile main file",
    extensions: &["shp"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
