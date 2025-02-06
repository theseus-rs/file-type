use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_1124114: FileFormat = FileFormat {
    id: 1_124_114,
    source_type: SourceType::Wikidata,
    name: "LandXML",
    extensions: &["ddf", "dem", "xml"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
