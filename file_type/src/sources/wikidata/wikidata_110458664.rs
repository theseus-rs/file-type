use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_110458664: FileFormat = FileFormat {
    id: 110_458_664,
    puid: "wikidata/110458664",
    name: "ERDAS Imagine Large Raster Spill File",
    extensions: &["ige"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
