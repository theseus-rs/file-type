use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_52426787: FileFormat = FileFormat {
    id: 52_426_787,
    source_type: SourceType::Wikidata,
    name: "XYWrite Document, version III+",
    extensions: &["xyp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
