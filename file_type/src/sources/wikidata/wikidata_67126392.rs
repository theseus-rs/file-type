use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_67126392: FileFormat = FileFormat {
    id: 67_126_392,
    source_type: SourceType::Wikidata,
    name: "Print Artist quote file format",
    extensions: &["qot"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
