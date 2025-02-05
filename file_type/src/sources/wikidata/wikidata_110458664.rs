use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_110458664: FileFormat = FileFormat {
    id: 110_458_664,
    source_type: SourceType::Wikidata,
    name: "ERDAS Imagine Large Raster Spill File",
    extensions: &["ige"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
