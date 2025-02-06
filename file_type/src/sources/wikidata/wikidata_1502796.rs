use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_1502796: FileFormat = FileFormat {
    id: 1_502_796,
    source_type: SourceType::Wikidata,
    name: "GeoTIFF",
    extensions: &["tif"],
    media_types: &["image/tiff"],
    signatures: &[],
    related_formats: &[],
};
