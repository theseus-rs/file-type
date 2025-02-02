use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28206446: FileFormat = FileFormat {
    id: 28_206_446,
    source_type: SourceType::Wikidata,
    name: "KiSS CEL 4-bit",
    extensions: &["cel"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
