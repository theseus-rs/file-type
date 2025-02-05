use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_114237069: FileFormat = FileFormat {
    id: 114_237_069,
    source_type: SourceType::Wikidata,
    name: "Visual C++ Definition format",
    extensions: &["def"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
