use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_131545334: FileFormat = FileFormat {
    id: 131_545_334,
    source_type: SourceType::Wikidata,
    name: "GeoArrow file format",
    extensions: &["arrow"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
