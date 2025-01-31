use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_278934: FileFormat = FileFormat {
    id: 278_934,
    puid: "wikidata/278934",
    name: "shapefile",
    extensions: &[
        "dbf", "dbf", "dbf", "shp", "shp", "shp", "shx", "shx", "shx",
    ],
    media_types: &[
        "application/vnd.dbf",
        "application/vnd.dbf",
        "application/vnd.dbf",
        "application/vnd.shp",
        "application/vnd.shp",
        "application/vnd.shp",
        "application/vnd.shx",
        "application/vnd.shx",
        "application/vnd.shx",
    ],
    internal_signatures: &[],
    related_formats: &[],
};
