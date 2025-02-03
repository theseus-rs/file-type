use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_278934: FileFormat = FileFormat {
    id: 278_934,
    source_type: SourceType::Wikidata,
    name: "shapefile",
    extensions: &["dbf", "shp", "shx"],
    media_types: &[
        "application/vnd.dbf",
        "application/vnd.shp",
        "application/vnd.shx",
    ],
    internal_signatures: &[],
    related_formats: &[],
};
