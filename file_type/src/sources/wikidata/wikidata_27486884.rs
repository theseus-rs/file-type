use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27486884: FileFormat = FileFormat {
    id: 27_486_884,
    source_type: SourceType::Wikidata,
    name: "Shapefile main file",
    extensions: &["shp"],
    media_types: &["application/octet-stream"],
    signatures: &[],
    related_formats: &[],
};
