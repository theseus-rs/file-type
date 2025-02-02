use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_67124713: FileFormat = FileFormat {
    id: 67_124_713,
    source_type: SourceType::Wikidata,
    name: "Print Artist postcard file format",
    extensions: &["pc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
