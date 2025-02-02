use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_110458664: FileFormat = FileFormat {
    id: 110_458_664,
    source_type: SourceType::Wikidata,
    name: "ERDAS Imagine Large Raster Spill File",
    extensions: &["ige"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
