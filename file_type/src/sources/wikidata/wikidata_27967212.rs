use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27967212: FileFormat = FileFormat {
    id: 27_967_212,
    source_type: SourceType::Wikidata,
    name: "RASTER Music Tracker module",
    extensions: &["rmt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
