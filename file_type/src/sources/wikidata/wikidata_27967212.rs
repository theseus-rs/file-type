use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27967212: FileFormat = FileFormat {
    id: 27_967_212,
    source_type: SourceType::Wikidata,
    name: "RASTER Music Tracker module",
    extensions: &["rmt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
