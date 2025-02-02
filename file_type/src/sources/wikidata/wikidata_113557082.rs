use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_113557082: FileFormat = FileFormat {
    id: 113_557_082,
    source_type: SourceType::Wikidata,
    name: "Creator Image format",
    extensions: &["cif"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
