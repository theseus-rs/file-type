use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_67384156: FileFormat = FileFormat {
    id: 67_384_156,
    source_type: SourceType::Wikidata,
    name: "SimLife Animal",
    extensions: &["anl"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
