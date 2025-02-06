use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206574: FileFormat = FileFormat {
    id: 28_206_574,
    source_type: SourceType::Wikidata,
    name: "MegaPaint BLD",
    extensions: &["bld"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
