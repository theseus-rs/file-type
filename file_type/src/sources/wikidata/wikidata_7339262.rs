use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_7339262: FileFormat = FileFormat {
    id: 7_339_262,
    source_type: SourceType::Wikidata,
    name: "RoadXML",
    extensions: &["rnd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
