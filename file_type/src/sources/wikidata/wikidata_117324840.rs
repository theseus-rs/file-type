use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_117324840: FileFormat = FileFormat {
    id: 117_324_840,
    source_type: SourceType::Wikidata,
    name: "Function Tree file",
    extensions: &["fp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
