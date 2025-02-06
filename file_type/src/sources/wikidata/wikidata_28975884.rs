use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28975884: FileFormat = FileFormat {
    id: 28_975_884,
    source_type: SourceType::Wikidata,
    name: "Geometry Data File",
    extensions: &["gdf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
