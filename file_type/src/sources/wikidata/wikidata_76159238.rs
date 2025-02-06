use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_76159238: FileFormat = FileFormat {
    id: 76_159_238,
    source_type: SourceType::Wikidata,
    name: "GDAL Raster Virtual Format",
    extensions: &["vrt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
