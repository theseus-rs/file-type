use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_81525646: FileFormat = FileFormat {
    id: 81_525_646,
    source_type: SourceType::Wikidata,
    name: "CorelDream 3D drawing",
    extensions: &["d3d"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
