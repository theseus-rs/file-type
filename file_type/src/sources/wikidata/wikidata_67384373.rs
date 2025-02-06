use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_67384373: FileFormat = FileFormat {
    id: 67_384_373,
    source_type: SourceType::Wikidata,
    name: "Crayola Art Studio graphic Art",
    extensions: &["art"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
