use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_114237069: FileFormat = FileFormat {
    id: 114_237_069,
    source_type: SourceType::Wikidata,
    name: "Visual C++ Definition format",
    extensions: &["def"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
