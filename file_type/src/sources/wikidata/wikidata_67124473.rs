use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_67124473: FileFormat = FileFormat {
    id: 67_124_473,
    source_type: SourceType::Wikidata,
    name: "Print Artist letterhead file format",
    extensions: &["lth"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
