use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_131545334: FileFormat = FileFormat {
    id: 131_545_334,
    source_type: SourceType::Wikidata,
    name: "GeoArrow file format",
    extensions: &["arrow"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
