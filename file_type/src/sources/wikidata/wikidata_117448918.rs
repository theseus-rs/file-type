use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_117448918: FileFormat = FileFormat {
    id: 117_448_918,
    source_type: SourceType::Wikidata,
    name: "B Source Code File",
    extensions: &["b"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
