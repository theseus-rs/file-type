use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_1502796: FileFormat = FileFormat {
    id: 1_502_796,
    source_type: SourceType::Wikidata,
    name: "GeoTIFF",
    extensions: &["tif"],
    media_types: &["image/tiff"],
    internal_signatures: &[],
    related_formats: &[],
};
