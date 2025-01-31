use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_76159238: FileFormat = FileFormat {
    id: 76_159_238,
    puid: "wikidata/76159238",
    name: "GDAL Raster Virtual Format",
    extensions: &["vrt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
