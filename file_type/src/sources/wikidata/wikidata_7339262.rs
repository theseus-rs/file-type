use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_7339262: FileFormat = FileFormat {
    id: 7_339_262,
    source_type: SourceType::Wikidata,
    name: "RoadXML",
    extensions: &["rnd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
