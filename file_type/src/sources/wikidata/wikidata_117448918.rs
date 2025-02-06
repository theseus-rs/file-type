use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_117448918: FileFormat = FileFormat {
    id: 117_448_918,
    source_type: SourceType::Wikidata,
    name: "B Source Code File",
    extensions: &["b"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
